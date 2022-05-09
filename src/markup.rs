mod html;
mod markdownv2;

use reqwest::Url;

use crate::types::{User, UserId};

pub use html::Html;
pub use markdownv2::MarkdownV2;

/// Allows formatting text according to one of the markup languages
/// supported by Telegram.
pub trait Markup {
    /// Applies the bold font style to the string.
    ///
    /// Passed string will not be automatically escaped because it can contain
    /// nested markup.
    fn bold(&self, s: &str) -> String;

    /// Applies the italic font style to the string.
    ///
    /// Passed string will not be automatically escaped because it can contain
    /// nested markup.
    fn italic(&self, s: &str) -> String;

    /// Applies the underline font style to the string.
    ///
    /// Passed string will not be automatically escaped because it can contain
    /// nested markup.
    fn underline(&self, s: &str) -> String;

    /// Applies the strikethrough font style to the string.
    ///
    /// Passed string will not be automatically escaped because it can contain
    /// nested markup.
    fn strikethrough(&self, s: &str) -> String;

    /// Builds an inline link with an anchor.
    ///
    /// Escapes the passed URL and the link text.
    fn link(&self, text: &str, url: Url) -> String;

    /// Builds an inline user mention link with an anchor.
    fn user_mention(&self, text: &str, user_id: UserId) -> String;

    fn user_mention_or_link(&self, user: &User) -> String;

    /// Formats the code block.
    ///
    /// This will escape formatting inside the provided string.
    fn code_block(&self, code: &str) -> String;

    /// Formats the code block with a specific language syntax.
    ///
    /// This will escape formatting inside the provided strings.
    ///
    /// Note that telegram generally ignores the language.
    fn code_block_with_lang(&self, code: &str, lang: &str) -> String;

    /// Formats the string as an inline code.
    ///
    /// This will escape formatting inside the provided string.
    fn code_inline(&self, s: &str) -> String;

    /// Escapes the string to be shown "as is" within the Telegram when applied
    /// appropriate [`ParseMode`].
    ///
    /// [`ParseMode`]: crate::types::ParseMode
    fn escape(&self, s: &str) -> String;

    /// Escapes the url so it can be used as a link in the inline link.
    ///
    /// Note: [`link`] automatically escapes the url, so this shouldn't normally
    /// be used.
    ///
    /// [`link`]: Markup::link
    fn escape_link_url(&self, u: Url) -> String;

    /// Escapes the url so it can be used as a code block.
    ///
    /// Note: [`code_block`] automatically escapes the code, so this shouldn't
    /// normally be used.
    ///
    /// [`code_block`]: Markup::code_block
    fn escape_code(&self, s: &str) -> String;
}

fn _assert_is_object_safe(_: &dyn Markup) {}
