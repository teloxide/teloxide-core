// see https://github.com/rust-lang/rust/issues/38832
// (for built ins there no warnings, but for (De)Serialize, there are)
#![allow(deprecated)]

use std::{
    convert::{TryFrom, TryInto},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::{
    markup::{Html, MarkdownV2, Markup},
    types::{User, UserId},
};

/// Formatting options.
///
/// The Bot API supports basic formatting for messages. You can use bold,
/// italic, underlined, strikethrough, and spoiler text, as well as inline links
/// and pre-formatted code in your bots' messages. Telegram clients will render
/// them accordingly. You can use either markdown-style or HTML-style
/// formatting.
///
/// Note that Telegram clients will display an **alert** to the user before
/// opening an inline link ('Open this link?' together with the full URL).
///
/// Message entities can be nested, providing following restrictions are met:
/// - If two entities have common characters then one of them is fully contained
///   inside another.
/// - bold, italic, underline, strikethrough, and spoiler entities can contain
///   and can be part of any other entities, except pre and code.
/// - All other entities can't contain each other.
///
/// Links `tg://user?id=<user_id>` can be used to mention a user by their ID
/// without using a username. Please note:
///
/// - These links will work **only** if they are used inside an inline link. For
///   example, they will not work, when used in an inline keyboard button or in
///   a message text.
/// - These mentions are only guaranteed to work if the user has contacted the
///   bot in the past, has sent a callback query to the bot via inline button or
///   is a member in the group where he was mentioned.
///
/// ## MarkdownV2 style
///
/// To use this mode, pass [`MarkdownV2`] in the `parse_mode` field.
/// Use the following syntax in your message:
/// ````text
/// *bold \*text*
/// _italic \*text_
/// __underline__
/// ~strikethrough~
/// ||spoiler||
/// *bold _italic bold ~italic bold strikethrough ||italic bold strikethrough spoiler||~ __underline italic bold___ bold*
/// [inline URL](http://www.example.com/)
/// [inline mention of a user](tg://user?id=123456789)
/// `inline fixed-width code`
/// ```
/// pre-formatted fixed-width code block
/// ```
/// ```rust
#[doc = "pre-formatted fixed-width code block written in the Rust programming language"]
/// ```
/// ````
///
/// Please note:
/// - Any character between 1 and 126 inclusively can be escaped anywhere with a
///   preceding '\' character, in which case it is treated as an ordinary
///   character and not a part of the markup.
/// - Inside `pre` and `code` entities, all '`‘ and ’\‘ characters must be
///   escaped with a preceding ’\' character.
/// - Inside `(...)` part of inline link definition, all ')‘ and ’\‘ must be
///   escaped with a preceding ’\' character.
/// - In all other places characters ’_‘, ’*‘, ’[‘, ’]‘, ’(‘, ’)‘, ’~‘, ’`‘,
///   ’>‘, ’#‘, ’+‘, ’+‘, ’-‘, ’|‘, ’{‘, ’}‘, ’.‘, ’!‘ must be escaped with the
///   preceding character ’\'.
/// - In case of ambiguity between `italic` and `underline` entities ‘__’ is
///   always greadily treated from left to right as beginning or end of
///   `underline` entity, so instead of `___italic underline___` use `___italic
///   underline_\r__`, where `\r` is a character with code `13`, which will be
///   ignored.
///
/// ## HTML style
///
/// To use this mode, pass [`Html`] in the `parse_mode` field.
/// The following tags are currently supported:
/// ````text
/// <b>bold</b>, <strong>bold</strong>
/// <i>italic</i>, <em>italic</em>
/// <u>underline</u>, <ins>underline</ins>
/// <s>strikethrough</s>, <strike>strikethrough</strike>, <del>strikethrough</del>
/// <span class="tg-spoiler">spoiler</span>, <tg-spoiler>spoiler</tg-spoiler>
/// <b>bold <i>italic bold <s>italic bold strikethrough <span class="tg-spoiler">italic bold strikethrough spoiler</span></s> <u>underline italic bold</u></i> bold</b>
/// <a href="http://www.example.com/">inline URL</a>
/// <a href="tg://user?id=123456789">inline mention of a user</a>
/// <code>inline fixed-width code</code>
/// <pre>pre-formatted fixed-width code block</pre>
#[doc = "<pre><code class=\"language-rust\">pre-formatted fixed-width code block written in the \
         Rust programming language</code></pre>"]
/// ````
/// 
/// Please note:
///
/// - Only the tags mentioned above are currently supported.
/// - All `<`, `>` and `&` symbols that are not a part of a tag or an HTML
///   entity must be replaced with the corresponding HTML entities (`<` with
///   `&lt;`, `>` with `&gt;` and `&` with `&amp;`).
/// - All numerical HTML entities are supported.
/// - The API currently supports only the following named HTML entities: `&lt;`,
///   `&gt;`, `&amp;` and `&quot;`.
/// - Use nested `pre` and `code` tags, to define programming language for `pre`
///   entity.
/// - Programming language can't be specified for standalone `code` tags.
///
/// ## Markdown style
///
/// This is a legacy mode, retained for backward compatibility. To use this
/// mode, pass [`Markdown`] in the `parse_mode` field.
/// Use the following syntax in your message:
/// ````text
/// *bold text*
/// _italic text_
/// [inline URL](http://www.example.com/)
/// [inline mention of a user](tg://user?id=123456789)
/// `inline fixed-width code`
/// ```
/// pre-formatted fixed-width code block
/// ```
/// ```rust
/// pre-formatted fixed-width code block written in the Rust programming language
/// ```
/// ````
/// 
/// Please note:
/// - Entities must not be nested, use parse mode [`MarkdownV2`] instead.
/// - There is no way to specify underline and strikethrough entities, use parse
///   mode [`MarkdownV2`] instead.
/// - To escape characters ’_‘, ’*‘, ’`‘, ’[‘ outside of an entity, prepend the
///   characters ’\' before them.
/// - Escaping inside entities is not allowed, so entity must be closed first
///   and reopened again: use `_snake_\__case_` for italic `snake_case` and
///   `*2*\**2=4*` for bold `2*2=4`.
///
/// [`MarkdownV2`]: ParseMode::MarkdownV2
/// [`Html`]: ParseMode::Html
/// [`Markdown`]: ParseMode::Markdown
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum ParseMode {
    MarkdownV2,
    #[serde(rename = "HTML")]
    Html,
    #[deprecated = "This is a legacy mode, retained for backward compatibility. Use `MarkdownV2` \
                    instead."]
    Markdown,
}

impl TryFrom<&str> for ParseMode {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let normalized = value.to_lowercase();
        match normalized.as_ref() {
            "html" => Ok(ParseMode::Html),
            "markdown" => Ok(ParseMode::Markdown),
            "markdownv2" => Ok(ParseMode::MarkdownV2),
            _ => Err(()),
        }
    }
}

impl TryFrom<String> for ParseMode {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl FromStr for ParseMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.try_into()
    }
}

impl Markup for ParseMode {
    fn bold(&self, s: &str) -> String {
        match self {
            Self::MarkdownV2 => MarkdownV2.bold(s),
            Self::Html => Html.bold(s),
            Self::Markdown => MarkdownV2.bold(s), // "best effort"
        }
    }

    fn italic(&self, s: &str) -> String {
        match self {
            Self::MarkdownV2 => MarkdownV2.italic(s),
            Self::Html => Html.italic(s),
            Self::Markdown => MarkdownV2.italic(s), // "best effort"
        }
    }

    fn underline(&self, s: &str) -> String {
        match self {
            Self::MarkdownV2 => MarkdownV2.underline(s),
            Self::Html => Html.underline(s),
            Self::Markdown => MarkdownV2.underline(s), // "best effort"
        }
    }

    fn strikethrough(&self, s: &str) -> String {
        match self {
            Self::MarkdownV2 => MarkdownV2.strikethrough(s),
            Self::Html => Html.strikethrough(s),
            Self::Markdown => MarkdownV2.strikethrough(s), // "best effort"
        }
    }

    fn link(&self, text: &str, url: reqwest::Url) -> String {
        match self {
            Self::MarkdownV2 => MarkdownV2.link(text, url),
            Self::Html => Html.link(text, url),
            Self::Markdown => MarkdownV2.link(text, url), // "best effort"
        }
    }

    fn user_mention(&self, text: &str, user_id: UserId) -> String {
        match self {
            Self::MarkdownV2 => MarkdownV2.user_mention(text, user_id),
            Self::Html => Html.user_mention(text, user_id),
            Self::Markdown => MarkdownV2.user_mention(text, user_id), // "best effort"
        }
    }

    fn user_mention_or_link(&self, user: &User) -> String {
        match self {
            Self::MarkdownV2 => MarkdownV2.user_mention_or_link(user),
            Self::Html => Html.user_mention_or_link(user),
            Self::Markdown => MarkdownV2.user_mention_or_link(user), // "best effort"
        }
    }

    fn code_block(&self, code: &str) -> String {
        match self {
            Self::MarkdownV2 => MarkdownV2.code_block(code),
            Self::Html => Html.code_block(code),
            Self::Markdown => MarkdownV2.code_block(code), // "best effort"
        }
    }

    fn code_block_with_lang(&self, code: &str, lang: &str) -> String {
        match self {
            Self::MarkdownV2 => MarkdownV2.code_block_with_lang(code, lang),
            Self::Html => Html.code_block_with_lang(code, lang),
            Self::Markdown => MarkdownV2.code_block_with_lang(code, lang), // "best effort"
        }
    }

    fn code_inline(&self, s: &str) -> String {
        match self {
            Self::MarkdownV2 => MarkdownV2.code_inline(s),
            Self::Html => Html.code_inline(s),
            Self::Markdown => MarkdownV2.code_inline(s), // "best effort"
        }
    }

    fn escape(&self, s: &str) -> String {
        match self {
            Self::MarkdownV2 => MarkdownV2.escape(s),
            Self::Html => Html.escape(s),
            Self::Markdown => MarkdownV2.escape(s), // "best effort"
        }
    }

    fn escape_link_url(&self, u: reqwest::Url) -> String {
        match self {
            Self::MarkdownV2 => MarkdownV2.escape_link_url(u),
            Self::Html => Html.escape_link_url(u),
            Self::Markdown => MarkdownV2.escape_link_url(u), // "best effort"
        }
    }

    fn escape_code(&self, s: &str) -> String {
        match self {
            Self::MarkdownV2 => MarkdownV2.escape_code(s),
            Self::Html => Html.escape_code(s),
            Self::Markdown => MarkdownV2.escape_code(s), // "best effort"
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(deprecated)]

    use super::*;

    #[test]
    fn html_serialization() {
        let expected_json = String::from(r#""HTML""#);
        let actual_json = serde_json::to_string(&ParseMode::Html).unwrap();

        assert_eq!(expected_json, actual_json)
    }

    #[test]
    fn markdown_serialization() {
        let expected_json = String::from(r#""Markdown""#);
        let actual_json = serde_json::to_string(&ParseMode::Markdown).unwrap();

        assert_eq!(expected_json, actual_json)
    }
}
