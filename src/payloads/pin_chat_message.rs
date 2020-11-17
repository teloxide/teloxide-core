// This file is auto generated by `cg` <https://github.com/teloxide/cg> (e634f65).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::ChatId;

impl_payload! {
    /// Use this method to pin a message in a group, a supergroup, or a channel. The bot must be an administrator in the chat for this to work and must have the 'can_pin_messages' admin right in the supergroup or 'can_edit_messages' admin right in the channel. Returns _True_ on success.
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub PinChatMessage (PinChatMessageSetters) => String {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: ChatId [into],
            /// Identifier of a message to pin
            pub message_id: i32,
        }
        optional {
            /// Pass True, if it is not necessary to send a notification to all chat members about the new pinned message. Notifications are always disabled in channels.
            pub disable_notification: bool,
        }
    }
}
