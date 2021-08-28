use serde_json::{self as json, Value};
use std::{
    fmt, fs,
    io::{BufRead, BufReader, Write},
};
use teloxide_core::types::Update;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

enum KeyPiece {
    Object(String),
    Array(u64),
}

impl fmt::Display for KeyPiece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyPiece::Object(k) => write!(f, ".{}", k),
            KeyPiece::Array(i) => write!(f, "[{}]", i),
        }
    }
}

// Vector is reversed
struct Key(Vec<KeyPiece>);

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for p in self.0.iter().rev() {
            p.fmt(f)?;
        }
        Ok(())
    }
}

struct Diff {
    path: Key,
    lhs: Value,
    rhs: Value,
}

// default lhs == rhs will return false when
// lhs = {key: null}
// rhs = {}
fn diff_null_aware(lhs: &Value, rhs: &Value) -> Vec<Diff> {
    use Value::*;

    fn is_null(val: &Value) -> bool {
        match val {
            Null => true,
            Array(a) if a.is_empty() => true,
            Object(m) if m.is_empty() => true,
            _ => false,
        }
    }

    fn drop_nulls(val: Value) -> Value {
        match val {
            Array(arr) => Array(arr.into_iter().map(drop_nulls).collect()),
            Object(map) => Object(
                map.into_iter()
                    .filter_map(|(k, v)| {
                        match v {
                            v if is_null(&v) => None,
                            Array(a) => Some(drop_nulls(Array(a))),
                            Object(m) => Some(drop_nulls(Object(m))),
                            other => Some(other),
                        }
                        .map(|v| (k, v))
                    })
                    .collect(),
            ),
            other => other,
        }
    }

    match (drop_nulls(lhs.clone()), drop_nulls(rhs.clone())) {
        (Null, Null) => Vec::new(),
        (Bool(lhs), Bool(rhs)) => (lhs != rhs)
            .then(|| {
                vec![Diff {
                    path: Key(Vec::new()),
                    lhs: Bool(lhs),
                    rhs: Bool(rhs),
                }]
            })
            .unwrap_or_default(),
        (Number(lhs), Number(rhs)) => (lhs != rhs)
            .then(|| {
                vec![Diff {
                    path: Key(Vec::new()),
                    lhs: Number(lhs),
                    rhs: Number(rhs),
                }]
            })
            .unwrap_or_default(),
        (String(lhs), String(rhs)) => (lhs != rhs)
            .then(|| {
                vec![Diff {
                    path: Key(Vec::new()),
                    lhs: String(lhs),
                    rhs: String(rhs),
                }]
            })
            .unwrap_or_default(),
        (Array(lhs), Array(rhs)) => lhs
            .into_iter()
            .zip(rhs)
            .enumerate()
            .map(|(i, (l, r))| {
                diff_null_aware(&l, &r).into_iter().map(move |mut d| {
                    d.path.0.push(KeyPiece::Array(i as _));
                    d
                })
            })
            .flatten()
            .collect(),
        (Object(lhs), Object(mut rhs)) => {
            let mut diffs = Vec::new();
            for (k, vl) in lhs {
                match rhs.remove(&k) {
                    Some(vr) => {
                        let d = diff_null_aware(&vl, &vr);
                        if !d.is_empty() {
                            diffs.extend(d);
                        }
                    }
                    None => {
                        diffs.push(Diff {
                            path: Key(vec![KeyPiece::Object(k)]),
                            lhs: vl,
                            rhs: Null,
                        });
                    }
                }
            }
            if rhs.len() > 0 {
                diffs.extend(rhs.into_iter().map(|(k, v)| Diff {
                    path: Key(vec![KeyPiece::Object(k)]),
                    lhs: Null,
                    rhs: v,
                }));
            }
            diffs
        }
        (lhs, rhs) => vec![Diff {
            path: Key(Vec::new()),
            lhs,
            rhs,
        }],
    }
}

#[test]
fn parse_updates() -> Result<(), Box<dyn std::error::Error>> {
    let file = BufReader::new(fs::File::open("tests/types/updates")?);
    let mut errs = 0u16;

    let mut stderr = StandardStream::stderr(termcolor::ColorChoice::Auto);
    let mut comment = String::new();
    for line in file.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        const COMMENT_PREFIX: &str = "# ";
        if line.starts_with(COMMENT_PREFIX) {
            if line.len() > COMMENT_PREFIX.len() {
                comment = line;
            } else {
                comment = String::new();
            }
            continue;
        }
        let value: Value = json::from_str(&line)?;
        let _ = json::from_value::<Update>(value.clone())
            .and_then(json::to_value)
            .map_err(|err| {
                stderr
                    .set_color(&ColorSpec::new().set_fg(Some(Color::Red)))
                    .unwrap();
                if !comment.is_empty() {
                    writeln!(stderr, "{} [ERROR]", comment).unwrap();
                }
                writeln!(stderr, "{:?}", err).unwrap();
                stderr.reset().unwrap();
            })
            .and_then(|ser| {
                let diffs = diff_null_aware(&ser, &value);
                if !diffs.is_empty() {
                    if !comment.is_empty() {
                        stderr
                            .set_color(&ColorSpec::new().set_fg(Some(Color::Red)))
                            .unwrap();
                        writeln!(stderr, "{} [ERROR]", comment).unwrap();
                        stderr.reset().unwrap();
                    }
                    for Diff { path, lhs, rhs } in diffs {
                        write!(stderr, "{} = ", path).unwrap();
                        stderr
                            .set_color(&ColorSpec::new().set_fg(Some(Color::Red)))
                            .unwrap();
                        write!(stderr, "{}/", lhs).unwrap();
                        stderr
                            .set_color(&ColorSpec::new().set_fg(Some(Color::Green)))
                            .unwrap();
                        writeln!(stderr, "{}", rhs).unwrap();
                        stderr.reset().unwrap();
                    }
                    Err(())
                } else {
                    stderr
                        .set_color(&ColorSpec::new().set_dimmed(true))
                        .unwrap();
                    writeln!(stderr, "{} [OK]", comment).unwrap();
                    stderr.reset().unwrap();
                    Ok(())
                }
            })
            .map_err(|()| errs += 1);
    }

    if errs > 0 {
        stderr.flush()?;
        Err("There were errors".into())
    } else {
        Ok(())
    }
}
