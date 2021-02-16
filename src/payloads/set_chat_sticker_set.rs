// This file is auto generated by `cg` <https://github.com/teloxide/cg> (8ee7ef2).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::{ChatId, True};

impl_payload! {
    /// Use this method to set a new group sticker set for a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Use the field _can\_set\_sticker\_set_ optionally returned in getChat requests to check if the bot can use this method. Returns _True_ on success.
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub SetChatStickerSet (SetChatStickerSetSetters) => True {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: ChatId [into],
            /// Name of the sticker set to be set as the group sticker set
            pub sticker_set_name: String [into],
        }
    }
}
