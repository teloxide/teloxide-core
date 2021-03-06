// This file is auto generated by [`cg`] from [`schema`].
//
// **DO NOT EDIT THIS FILE**,
//
// Edit `cg` or `schema` instead.
//
// [cg]: https://github.com/teloxide/cg
// [`schema`]: https://github.com/WaffleLapkin/tg-methods-schema
use serde::Serialize;

use crate::types::{InlineKeyboardMarkup, True};

impl_payload! {
    /// Use this method to edit only the reply markup of messages. On success, _True_ is returned.
    ///
    /// See also: [`EditMessageReplyMarkup`](crate::payloads::EditMessageReplyMarkup)
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub EditMessageReplyMarkupInline (EditMessageReplyMarkupInlineSetters) => True {
        required {
            /// Identifier of the inline message
            pub inline_message_id: String [into],
        }
        optional {
            /// A JSON-serialized object for an [inline keyboard].
            ///
            /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
            pub reply_markup: InlineKeyboardMarkup,
        }
    }
}
