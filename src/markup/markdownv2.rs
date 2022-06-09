use crate::{
    markup::Markup,
    types::{User, UserId},
};
use aho_corasick::{AhoCorasick, Match};
use once_cell::sync::Lazy;
use reqwest::Url;

/// Allows formatting text according "MarkdownV2" Telegram markup language. See
/// [specification].
///
/// [specification]: https://core.telegram.org/bots/api#markdownv2-style
pub struct MarkdownV2;

impl Markup for MarkdownV2 {
    fn bold(&self, s: &str) -> String {
        format!("*{}*", s)
    }

    fn italic(&self, s: &str) -> String {
        if s.starts_with("__") && s.ends_with("__") {
            format!(r"_{}\r__", &s[..s.len() - 1])
        } else {
            format!("_{s}_")
        }
    }

    fn underline(&self, s: &str) -> String {
        // In case of ambiguity between italic and underline entities
        // ‘__’ is always greedily treated from left to right as beginning or end of
        // underline entity, so instead of ___italic underline___ we should use
        // ___italic underline_\r__, where \r is a character with code 13, which
        // will be ignored.
        if s.starts_with('_') && s.ends_with('_') {
            format!(r"__{s}\r__")
        } else {
            format!("__{s}__")
        }
    }

    fn strikethrough(&self, s: &str) -> String {
        format!("~{s}~")
    }

    fn link(&self, text: &str, url: Url) -> String {
        let url = self.escape_link_url(url);

        format!("[{text}]({url})")
    }

    fn user_mention(&self, text: &str, user_id: UserId) -> String {
        self.link(text, user_id.url())
    }

    fn user_mention_or_link(&self, user: &User) -> String {
        match user.mention() {
            Some(mention) => mention,
            None => self.link(&user.full_name(), user.url()),
        }
    }

    fn code_block(&self, code: &str) -> String {
        let code = self.escape_code(code);

        format!("```\n{code}\n```")
    }

    fn code_block_with_lang(&self, code: &str, lang: &str) -> String {
        let code = self.escape_code(code);
        let lang = self.escape(lang);

        format!("```{lang}\n{code}\n```")
    }

    fn code_inline(&self, s: &str) -> String {
        let s = self.escape_code(s);

        format!("`{s}`")
    }

    fn escape(&self, s: &str) -> String {
        static SEARCHER: Lazy<AhoCorasick> = Lazy::new(|| {
            AhoCorasick::new_auto_configured(&[
                r"\", "_", "*", "[", "]", "(", ")", "~", "`", ">", "#", "+", "-", "=", "|", "{",
                "}", ".", "!",
            ])
        });

        let mut dst = String::with_capacity(s.len());
        SEARCHER.replace_all_with(s, &mut dst, precede_with_back_slash);

        dst
    }

    fn escape_link_url(&self, u: Url) -> String {
        let s = u.as_str();

        static SEARCHER: Lazy<AhoCorasick> =
            Lazy::new(|| AhoCorasick::new_auto_configured(&[r"\", ")"]));

        let mut dst = String::with_capacity(s.len());
        SEARCHER.replace_all_with(s, &mut dst, precede_with_back_slash);

        dst
    }

    fn escape_code(&self, s: &str) -> String {
        static SEARCHER: Lazy<AhoCorasick> =
            Lazy::new(|| AhoCorasick::new_auto_configured(&[r"\", "`"]));

        let mut dst = String::with_capacity(s.len());
        SEARCHER.replace_all_with(s, &mut dst, precede_with_back_slash);

        dst
    }
}

fn precede_with_back_slash(_: &Match, replaced: &str, dst: &mut String) -> bool {
    dst.push('\\');
    dst.push_str(replaced);

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::User;

    #[test]
    fn test_bold() {
        assert_eq!(MarkdownV2.bold(" foobar "), "* foobar *");
        assert_eq!(MarkdownV2.bold(" _foobar_ "), "* _foobar_ *");
        assert_eq!(MarkdownV2.bold("~(`foobar`)~"), "*~(`foobar`)~*");
    }

    #[test]
    fn test_italic() {
        assert_eq!(MarkdownV2.italic(" foobar "), "_ foobar _");
        assert_eq!(MarkdownV2.italic("*foobar*"), "_*foobar*_");
        assert_eq!(MarkdownV2.italic("~(foobar)~"), "_~(foobar)~_");
    }

    #[test]
    fn test_underline() {
        assert_eq!(MarkdownV2.underline(" foobar "), "__ foobar __");
        assert_eq!(MarkdownV2.underline("*foobar*"), "__*foobar*__");
        assert_eq!(MarkdownV2.underline("~(foobar)~"), "__~(foobar)~__");
    }

    #[test]
    fn test_strike() {
        assert_eq!(MarkdownV2.strikethrough(" foobar "), "~ foobar ~");
        assert_eq!(MarkdownV2.strikethrough("*foobar*"), "~*foobar*~");
        assert_eq!(MarkdownV2.strikethrough("*(foobar)*"), "~*(foobar)*~");
    }

    #[test]
    fn test_italic_with_underline() {
        assert_eq!(
            MarkdownV2.underline(MarkdownV2.italic("foobar").as_str()),
            r"___foobar_\r__"
        );
        assert_eq!(
            MarkdownV2.italic(MarkdownV2.underline("foobar").as_str()),
            r"___foobar_\r__"
        );
    }

    #[test]
    fn test_link() {
        assert_eq!(
            MarkdownV2.link(
                "google",
                "https://www.google.com?q=(%60foobar%60)".parse().unwrap(),
            ),
            r"[google](https://www.google.com/?q=(%60foobar%60\))",
        );
    }

    #[test]
    fn test_user_mention() {
        assert_eq!(
            MarkdownV2.user_mention("pwner666", UserId(123_456_789)),
            "[pwner666](tg://user/?id=123456789)"
        );
    }

    #[test]
    fn test_code_block() {
        assert_eq!(
            MarkdownV2.code_block("pre-'formatted'\nfixed-width \\code `block`"),
            "```\npre-'formatted'\nfixed-width \\\\code \\`block\\`\n```"
        );
    }

    #[test]
    fn test_code_block_with_lang() {
        assert_eq!(
            MarkdownV2
                .code_block_with_lang("pre-'formatted'\nfixed-width \\code `block`", "[python]"),
            "```\\[python\\]\npre-'formatted'\nfixed-width \\\\code \\`block\\`\n```"
        );
    }

    #[test]
    fn test_code_inline() {
        assert_eq!(
            MarkdownV2.code_inline(" let x = (1, 2, 3); "),
            "` let x = (1, 2, 3); `"
        );
        assert_eq!(
            MarkdownV2.code_inline("<html>foo</html>"),
            "`<html>foo</html>`"
        );
        assert_eq!(
            MarkdownV2.code_inline(r" `(code inside code \ )` "),
            r"` \`(code inside code \\ )\` `"
        );
    }

    #[test]
    fn test_escape() {
        assert_eq!(MarkdownV2.escape("* foobar *"), r"\* foobar \*");
        assert_eq!(
            MarkdownV2.escape(r"_ * [ ] ( ) ~ \ ` > # + - = | { } . !"),
            r"\_ \* \[ \] \( \) \~ \\ \` \> \# \+ \- \= \| \{ \} \. \!",
        );
    }

    #[test]
    fn test_escape_link_url() {
        assert_eq!(
            MarkdownV2.escape_link_url(
                r"https://en.wikipedia.org/wiki/Development+(Software)"
                    .parse()
                    .unwrap()
            ),
            r"https://en.wikipedia.org/wiki/Development+(Software\)"
        );
        assert_eq!(
            MarkdownV2.escape_link_url(r"https://en.wikipedia.org/wiki/`".parse().unwrap()),
            r"https://en.wikipedia.org/wiki/%60"
        );
        assert_eq!(
            MarkdownV2.escape_link_url(r"https://example.com/_*[]()~`#+-=|{}.!\".parse().unwrap()),
            r"https://example.com/_*[](\)~%60#+-=|{}.!\\"
        );
    }

    #[test]
    fn test_escape_code() {
        assert_eq!(
            MarkdownV2.escape_code(r"` \code inside the code\ `"),
            r"\` \\code inside the code\\ \`"
        );
        assert_eq!(
            MarkdownV2.escape_code(r"_*[]()~`#+-=|{}.!\"),
            r"_*[]()~\`#+-=|{}.!\\"
        );
    }

    #[test]
    fn user_mention_link() {
        let user_with_username = User {
            id: UserId(0),
            is_bot: false,
            first_name: "".to_string(),
            last_name: None,
            username: Some("abcd".to_string()),
            language_code: None,
        };
        assert_eq!(
            MarkdownV2.user_mention_or_link(&user_with_username),
            "@abcd"
        );
        let user_without_username = User {
            id: UserId(123_456_789),
            is_bot: false,
            first_name: "Name".to_string(),
            last_name: None,
            username: None,
            language_code: None,
        };
        assert_eq!(
            MarkdownV2.user_mention_or_link(&user_without_username),
            r#"[Name](tg://user/?id=123456789)"#
        )
    }
}
