// This file is auto generated by `cg` <https://github.com/teloxide/cg> (e634f65).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::{ChatId, True};

impl_payload! {
    /// Use this method to set a custom title for an administrator in a supergroup promoted by the bot. Returns _True_on success.
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub SetChatAdministratorCustomTitle (SetChatAdministratorCustomTitleSetters) => True {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: ChatId [into],
            /// Unique identifier of the target user
            pub user_id: i32,
            /// New custom title for the administrator; 0-16 characters, emoji are not allowed
            pub custom_title: String [into],
        }
    }
}
