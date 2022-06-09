use aho_corasick::{AhoCorasick, Match};
use once_cell::sync::Lazy;
use reqwest::Url;

use crate::{
    markup::Markup,
    types::{User, UserId},
};

/// Allows formatting text according "HTML" Telegram markup language. See
/// [specification].
///
/// [specification]: https://core.telegram.org/bots/api#html-style
pub struct Html;

impl Markup for Html {
    fn bold(&self, s: &str) -> String {
        format!("<b>{s}</b>")
    }

    fn italic(&self, s: &str) -> String {
        format!("<i>{s}</i>")
    }

    fn underline(&self, s: &str) -> String {
        format!("<u>{s}</u>")
    }

    fn strikethrough(&self, s: &str) -> String {
        format!("<s>{s}</s>")
    }

    fn link(&self, text: &str, url: Url) -> String {
        let text = self.escape(text);
        let url = self.escape(url.as_str());

        format!("<a href=\"{url}\">{text}</a>")
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
        let code = self.escape(code);

        format!("<pre>{code}</pre>")
    }

    fn code_block_with_lang(&self, code: &str, lang: &str) -> String {
        static SEARCHER: Lazy<AhoCorasick> =
            Lazy::new(|| AhoCorasick::new_auto_configured(&["&", "<", ">", "\""]));

        let mut language = String::with_capacity(lang.len());
        SEARCHER.replace_all_with(lang, &mut language, html_replacement);

        let code = self.escape(code);

        format!("<pre><code class=\"language-{language}\">{code}</code></pre>")
    }

    fn code_inline(&self, s: &str) -> String {
        let s = self.escape(s);

        format!("<code>{s}</code>")
    }

    fn escape(&self, s: &str) -> String {
        static SEARCHER: Lazy<AhoCorasick> =
            Lazy::new(|| AhoCorasick::new_auto_configured(&["&", "<", ">"]));

        let mut dst = String::with_capacity(s.len());
        SEARCHER.replace_all_with(s, &mut dst, html_replacement);

        dst
    }

    fn escape_link_url(&self, u: Url) -> String {
        self.escape(u.as_str())
    }

    fn escape_code(&self, s: &str) -> String {
        self.escape(s)
    }
}

fn html_replacement(mat: &Match, _: &str, dst: &mut String) -> bool {
    let replacement = match mat.pattern() {
        0 => "&amp;",
        1 => "&lt;",
        2 => "&gt;",
        3 => "&quot;",
        _ => "unreachable",
    };

    dst.push_str(replacement);

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bold() {
        assert_eq!(Html.bold(" foobar "), "<b> foobar </b>");
        assert_eq!(Html.bold(" <i>foobar</i> "), "<b> <i>foobar</i> </b>");
        assert_eq!(Html.bold("<s>(`foobar`)</s>"), "<b><s>(`foobar`)</s></b>");
    }

    #[test]
    fn test_italic() {
        assert_eq!(Html.italic(" foobar "), "<i> foobar </i>");
        assert_eq!(Html.italic(" <b>foobar</b> "), "<i> <b>foobar</b> </i>");
        assert_eq!(Html.italic("<s>(`foobar`)</s>"), "<i><s>(`foobar`)</s></i>");
    }

    #[test]
    fn test_underline() {
        assert_eq!(Html.underline(" foobar "), "<u> foobar </u>");
        assert_eq!(Html.underline(" <b>foobar</b> "), "<u> <b>foobar</b> </u>");
        assert_eq!(
            Html.underline("<s>(`foobar`)</s>"),
            "<u><s>(`foobar`)</s></u>"
        );
    }

    #[test]
    fn test_strike() {
        assert_eq!(Html.strikethrough(" foobar "), "<s> foobar </s>");
        assert_eq!(
            Html.strikethrough(" <b>foobar</b> "),
            "<s> <b>foobar</b> </s>"
        );
        assert_eq!(
            Html.strikethrough("<b>(`foobar`)</b>"),
            "<s><b>(`foobar`)</b></s>"
        );
    }

    #[test]
    fn test_link() {
        assert_eq!(
            Html.link(
                "<google>",
                "https://www.google.com/?q=foo&l=ru".parse().unwrap()
            ),
            "<a href=\"https://www.google.com/?q=foo&amp;l=ru\">&lt;google&gt;</a>",
        );
    }

    #[test]
    fn test_user_mention() {
        assert_eq!(
            Html.user_mention("<pwner666>", UserId(123_456_789)),
            "<a href=\"tg://user/?id=123456789\">&lt;pwner666&gt;</a>",
        );
    }

    #[test]
    fn test_code_block() {
        assert_eq!(
            Html.code_block("<p>pre-'formatted'\n & fixed-width \\code `block`</p>"),
            "<pre>&lt;p&gt;pre-'formatted'\n &amp; fixed-width \\code `block`&lt;/p&gt;</pre>"
        );
    }

    #[test]
    fn test_code_block_with_lang() {
        assert_eq!(
            Html.code_block_with_lang(
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
            Html.code_inline("<span class=\"foo\">foo & bar</span>"),
            "<code>&lt;span class=\"foo\"&gt;foo &amp; bar&lt;/span&gt;</code>",
        );
    }

    #[test]
    fn test_escape() {
        assert_eq!(
            Html.escape("  <title>Foo & Bar</title>   "),
            "  &lt;title&gt;Foo &amp; Bar&lt;/title&gt;   "
        );
        assert_eq!(
            Html.escape("<p>你好 & 再見</p>"),
            "&lt;p&gt;你好 &amp; 再見&lt;/p&gt;"
        );
        assert_eq!(Html.escape("'foo\""), "'foo\"");
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
        assert_eq!(Html.user_mention_or_link(&user_with_username), "@abcd");
        let user_without_username = User {
            id: UserId(123_456_789),
            is_bot: false,
            first_name: "Name".to_string(),
            last_name: None,
            username: None,
            language_code: None,
        };
        assert_eq!(
            Html.user_mention_or_link(&user_without_username),
            r#"<a href="tg://user/?id=123456789">Name</a>"#
        )
    }
}
