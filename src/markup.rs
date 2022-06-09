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
    #[must_use = "This function does not mutate its input, it returns a newly created string"]
    fn bold(&self, s: &str) -> String;

    /// Applies the italic font style to the string.
    ///
    /// Passed string will not be automatically escaped because it can contain
    /// nested markup.
    #[must_use = "This function does not mutate its input, it returns a newly created string"]
    fn italic(&self, s: &str) -> String;

    /// Applies the underline font style to the string.
    ///
    /// Passed string will not be automatically escaped because it can contain
    /// nested markup.
    #[must_use = "This function does not mutate its input, it returns a newly created string"]
    fn underline(&self, s: &str) -> String;

    /// Applies the strikethrough font style to the string.
    ///
    /// Passed string will not be automatically escaped because it can contain
    /// nested markup.
    #[must_use = "This function does not mutate its input, it returns a newly created string"]
    fn strikethrough(&self, s: &str) -> String;

    /// Builds an inline link with an anchor.
    ///
    /// Escapes the passed URL and the link text.
    #[must_use = "This function does not mutate its input, it returns a newly created string"]
    fn link(&self, text: &str, url: Url) -> String;

    /// Builds an inline user mention link with an anchor.
    #[must_use = "This function does not mutate its input, it returns a newly created string"]
    fn user_mention(&self, text: &str, user_id: UserId) -> String {
        self.link(text, user_id.url())
    }

    #[must_use = "This function does not mutate its input, it returns a newly created string"]
    fn user_mention_or_link(&self, user: &User) -> String {
        match user.mention() {
            Some(mention) => mention,
            None => {
                let name = user.full_name();
                let name = self.escape(&name);

                self.link(&name, user.url())
            }
        }
    }

    /// Formats the code block.
    ///
    /// This will escape formatting inside the provided string.
    #[must_use = "This function does not mutate its input, it returns a newly created string"]
    fn code_block(&self, code: &str) -> String;

    /// Formats the code block with a specific language syntax.
    ///
    /// This will escape formatting inside the provided strings.
    ///
    /// Note that telegram generally ignores the language.
    #[must_use = "This function does not mutate its input, it returns a newly created string"]
    fn code_block_with_lang(&self, code: &str, lang: &str) -> String;

    /// Formats the string as an inline code.
    ///
    /// This will escape formatting inside the provided string.
    #[must_use = "This function does not mutate its input, it returns a newly created string"]
    fn code_inline(&self, s: &str) -> String;

    /// Escapes the string to be shown "as is" within the Telegram when applied
    /// appropriate [`ParseMode`].
    ///
    /// [`ParseMode`]: crate::types::ParseMode
    #[must_use = "This function does not mutate its input, it returns a newly created string"]
    fn escape(&self, s: &str) -> String;

    /// Escapes the url so it can be used as a link in the inline link.
    ///
    /// Note: [`link`] automatically escapes the url, so this shouldn't normally
    /// be used.
    ///
    /// [`link`]: Markup::link
    #[must_use = "This function does not mutate its input, it returns a newly created string"]
    fn escape_link_url(&self, u: Url) -> String;

    /// Escapes the url so it can be used as a code block.
    ///
    /// Note: [`code_block`] automatically escapes the code, so this shouldn't
    /// normally be used.
    ///
    /// [`code_block`]: Markup::code_block
    #[must_use = "This function does not mutate its input, it returns a newly created string"]
    fn escape_code(&self, s: &str) -> String;
}

fn _assert_is_object_safe(_: &dyn Markup) {}
