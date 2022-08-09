//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::{Recipient, True};

impl_payload! {
    /// Use this method to change the title of a chat. Titles can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns _True_ on success.
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub SetChatTitle (SetChatTitleSetters) => True {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: Recipient [into],
            /// New chat title, 1-255 characters
            pub title: String [into],
        }
    }
}
