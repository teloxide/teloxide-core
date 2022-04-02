//! Utils for working with the [HTML message style][spec].
//!
//! [spec]: https://core.telegram.org/bots/api#html-style

use reqwest::Url;

use crate::types::{User, UserId};

/// Applies the bold font style to the string.
///
/// Passed string will not be automatically escaped because it can contain
/// nested markup.
pub fn bold(s: &str) -> String {
    format!("<b>{s}</b>")
}

/// Applies the italic font style to the string.
///
/// Passed string will not be automatically escaped because it can contain
/// nested markup.
pub fn italic(s: &str) -> String {
    format!("<i>{s}</i>")
}

/// Applies the underline font style to the string.
///
/// Passed string will not be automatically escaped because it can contain
/// nested markup.
pub fn underline(s: &str) -> String {
    format!("<u>{s}</u>")
}

/// Applies the strikethrough font style to the string.
///
/// Passed string will not be automatically escaped because it can contain
/// nested markup.
pub fn strikethrough(s: &str) -> String {
    format!("<s>{s}</s>")
}

/// Builds an inline link with an anchor.
///
/// Escapes the passed URL and the link text.
pub fn link(text: &str, url: Url) -> String {
    format!("<a href=\"{}\">{}</a>", escape(url.as_str()), escape(text))
}

/// Builds an inline user mention link with an anchor.
pub fn user_mention(user_id: UserId, text: &str) -> String {
    // FIXME: use user_id.url()
    link(text, format!("tg://user?id={user_id}").parse().unwrap())
}

/// Formats the code block.
///
/// Escapes HTML characters inside the block.
pub fn code_block(code: &str) -> String {
    format!("<pre>{}</pre>", escape(code))
}

/// Formats the code block with a specific language syntax.
///
/// Escapes HTML characters inside the block.
pub fn code_block_with_lang(code: &str, lang: &str) -> String {
    format!(
        "<pre><code class=\"language-{}\">{}</code></pre>",
        escape(lang).replace('"', "&quot;"),
        escape(code)
    )
}

/// Formats the string as an inline code.
///
/// Escapes HTML characters inside the block.
pub fn code_inline(s: &str) -> String {
    format!("<code>{}</code>", escape(s))
}

/// Escapes the string to be shown "as is" within the Telegram HTML message
/// style.
///
/// Does not escape ' and " characters (as should be for usual HTML), because
/// they shoudn't be escaped by the [spec].
///
/// [spec]: https://core.telegram.org/bots/api#html-style
pub fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

pub fn user_mention_or_link(user: &User) -> String {
    match user.mention() {
        Some(mention) => mention,
        None => link(&user.full_name(), user.url()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bold() {
        assert_eq!(bold(" foobar "), "<b> foobar </b>");
        assert_eq!(bold(" <i>foobar</i> "), "<b> <i>foobar</i> </b>");
        assert_eq!(bold("<s>(`foobar`)</s>"), "<b><s>(`foobar`)</s></b>");
    }

    #[test]
    fn test_italic() {
        assert_eq!(italic(" foobar "), "<i> foobar </i>");
        assert_eq!(italic(" <b>foobar</b> "), "<i> <b>foobar</b> </i>");
        assert_eq!(italic("<s>(`foobar`)</s>"), "<i><s>(`foobar`)</s></i>");
    }

    #[test]
    fn test_underline() {
        assert_eq!(underline(" foobar "), "<u> foobar </u>");
        assert_eq!(underline(" <b>foobar</b> "), "<u> <b>foobar</b> </u>");
        assert_eq!(underline("<s>(`foobar`)</s>"), "<u><s>(`foobar`)</s></u>");
    }

    #[test]
    fn test_strike() {
        assert_eq!(strikethrough(" foobar "), "<s> foobar </s>");
        assert_eq!(strikethrough(" <b>foobar</b> "), "<s> <b>foobar</b> </s>");
        assert_eq!(
            strikethrough("<b>(`foobar`)</b>"),
            "<s><b>(`foobar`)</b></s>"
        );
    }

    #[test]
    fn test_link() {
        assert_eq!(
            link(
                "<google>",
                "https://www.google.com/?q=foo&l=ru".parse().unwrap()
            ),
            "<a href=\"https://www.google.com/?q=foo&amp;l=ru\">&lt;google&gt;</a>",
        );
    }

    #[test]
    fn test_user_mention() {
        assert_eq!(
            user_mention(UserId(123_456_789), "<pwner666>"),
            "<a href=\"tg://user?id=123456789\">&lt;pwner666&gt;</a>",
        );
    }

    #[test]
    fn test_code_block() {
        assert_eq!(
            code_block("<p>pre-'formatted'\n & fixed-width \\code `block`</p>"),
            "<pre>&lt;p&gt;pre-'formatted'\n &amp; fixed-width \\code `block`&lt;/p&gt;</pre>"
        );
    }

    #[test]
    fn test_code_block_with_lang() {
        assert_eq!(
            code_block_with_lang(
                "<p>pre-'formatted'\n & fixed-width \\code `block`</p>",
                "<html>\""
            ),
            concat!(
                "<pre><code class=\"language-&lt;html&gt;&quot;\">",
                "&lt;p&gt;pre-'formatted'\n &amp; fixed-width \\code `block`&lt;/p&gt;",
                "</code></pre>",
            )
        );
    }

    #[test]
    fn test_code_inline() {
        assert_eq!(
            code_inline("<span class=\"foo\">foo & bar</span>"),
            "<code>&lt;span class=\"foo\"&gt;foo &amp; bar&lt;/span&gt;</code>",
        );
    }

    #[test]
    fn test_escape() {
        assert_eq!(
            escape("  <title>Foo & Bar</title>   "),
            "  &lt;title&gt;Foo &amp; Bar&lt;/title&gt;   "
        );
        assert_eq!(
            escape("<p>你好 & 再見</p>"),
            "&lt;p&gt;你好 &amp; 再見&lt;/p&gt;"
        );
        assert_eq!(escape("'foo\""), "'foo\"");
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
        assert_eq!(user_mention_or_link(&user_with_username), "@abcd");
        let user_without_username = User {
            id: UserId(123_456_789),
            is_bot: false,
            first_name: "Name".to_string(),
            last_name: None,
            username: None,
            language_code: None,
        };
        assert_eq!(
            user_mention_or_link(&user_without_username),
            r#"<a href="tg://user/?id=123456789">Name</a>"#
        )
    }
}
