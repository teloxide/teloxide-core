//! Telergam API types.

pub use allowed_update::*;
pub use animation::*;
pub use audio::*;
pub use bot_command::*;
pub use bot_command_scope::*;
pub use callback_game::*;
pub use callback_query::*;
pub use chat::*;
pub use chat_action::*;
pub use chat_id::*;
pub use chat_invite_link::*;
pub use chat_join_request::*;
pub use chat_location::*;
pub use chat_member::*;
pub use chat_member_updated::*;
pub use chat_permissions::*;
pub use chat_photo::*;
pub use chat_type::*;
pub use chosen_inline_result::*;
pub use contact::*;
pub use dice::*;
pub use dice_emoji::*;
pub use document::*;
pub use encrypted_credentials::*;
pub use encrypted_passport_element::*;
pub use file::*;
pub use force_reply::*;
pub use game::*;
pub use game_high_score::*;
pub use inline_keyboard_button::*;
pub use inline_keyboard_markup::*;
pub use inline_query::*;
pub use inline_query_result::*;
pub use inline_query_result_article::*;
pub use inline_query_result_audio::*;
pub use inline_query_result_cached_audio::*;
pub use inline_query_result_cached_document::*;
pub use inline_query_result_cached_gif::*;
pub use inline_query_result_cached_mpeg4_gif::*;
pub use inline_query_result_cached_photo::*;
pub use inline_query_result_cached_sticker::*;
pub use inline_query_result_cached_video::*;
pub use inline_query_result_cached_voice::*;
pub use inline_query_result_contact::*;
pub use inline_query_result_document::*;
pub use inline_query_result_game::*;
pub use inline_query_result_gif::*;
pub use inline_query_result_location::*;
pub use inline_query_result_mpeg4_gif::*;
pub use inline_query_result_photo::*;
pub use inline_query_result_venue::*;
pub use inline_query_result_video::*;
pub use inline_query_result_voice::*;
pub use input_file::*;
pub use input_media::*;
pub use input_message_content::*;
pub use input_sticker::*;
pub use invoice::*;
pub use keyboard_button::*;
pub use keyboard_button_poll_type::*;
pub use label_price::*;
pub use location::*;
pub use login_url::*;
pub use mask_position::*;
pub use me::*;
pub use message::*;
pub use message_auto_delete_timer_changed::*;
pub use message_entity::*;
pub use message_id::*;
pub use order_info::*;
pub use parse_mode::*;
pub use passport_data::*;
pub use passport_element_error::*;
pub use passport_file::*;
pub use photo_size::*;
pub use poll::*;
pub use poll_answer::*;
pub use poll_type::*;
pub use pre_checkout_query::*;
pub use proximity_alert_triggered::*;
pub use reply_keyboard_markup::*;
pub use reply_keyboard_remove::*;
pub use reply_markup::*;
pub use response_parameters::*;
pub use shipping_address::*;
pub use shipping_option::*;
pub use shipping_query::*;
pub use sticker::*;
pub use sticker_set::*;
pub use successful_payment::*;
pub use target_message::*;
pub use unit_false::*;
pub use unit_true::*;
pub use update::*;
pub use user::*;
pub use user_profile_photos::*;
pub use venue::*;
pub use video::*;
pub use video_note::*;
pub use voice::*;
pub use voice_chat_ended::*;
pub use voice_chat_participants_invited::*;
pub use voice_chat_scheduled::*;
pub use voice_chat_started::*;
pub use webhook_info::*;

mod allowed_update;
mod animation;
mod audio;
mod bot_command;
mod bot_command_scope;
mod callback_game;
mod callback_query;
mod chat;
mod chat_action;
mod chat_id;
mod chat_invite_link;
mod chat_join_request;
mod chat_location;
mod chat_member;
mod chat_member_updated;
mod chat_permissions;
mod chat_photo;
mod chat_type;
mod chosen_inline_result;
mod contact;
mod dice;
mod dice_emoji;
mod document;
mod file;
mod force_reply;
mod game;
mod game_high_score;
mod inline_keyboard_button;
mod inline_keyboard_markup;
mod input_file;
mod input_media;
mod input_message_content;
mod input_sticker;
mod invoice;
mod keyboard_button;
mod keyboard_button_poll_type;
mod label_price;
mod location;
mod login_url;
mod mask_position;
mod me;
mod message;
mod message_auto_delete_timer_changed;
mod message_entity;
mod message_id;
mod order_info;
mod parse_mode;
mod photo_size;
mod poll;
mod poll_answer;
mod poll_type;
mod pre_checkout_query;
mod proximity_alert_triggered;
mod reply_keyboard_markup;
mod reply_keyboard_remove;
mod reply_markup;
mod response_parameters;
mod shipping_address;
mod shipping_option;
mod shipping_query;
mod sticker;
mod sticker_set;
mod successful_payment;
mod target_message;
mod unit_false;
mod unit_true;
mod update;
mod user;
mod user_profile_photos;
mod venue;
mod video;
mod video_note;
mod voice;
mod voice_chat_ended;
mod voice_chat_participants_invited;
mod voice_chat_scheduled;
mod voice_chat_started;
mod webhook_info;

mod inline_query;
mod inline_query_result;
mod inline_query_result_article;
mod inline_query_result_audio;
mod inline_query_result_cached_audio;
mod inline_query_result_cached_document;
mod inline_query_result_cached_gif;
mod inline_query_result_cached_mpeg4_gif;
mod inline_query_result_cached_photo;
mod inline_query_result_cached_sticker;
mod inline_query_result_cached_video;
mod inline_query_result_cached_voice;
mod inline_query_result_contact;
mod inline_query_result_document;
mod inline_query_result_game;
mod inline_query_result_gif;
mod inline_query_result_location;
mod inline_query_result_mpeg4_gif;
mod inline_query_result_photo;
mod inline_query_result_venue;
mod inline_query_result_video;
mod inline_query_result_voice;

mod encrypted_credentials;
mod encrypted_passport_element;
mod passport_data;
mod passport_element_error;
mod passport_file;

pub use non_telegram_types::{country_code::*, currency::*, semiparsed_vec::*, until_date::*};
mod non_telegram_types {
    pub(super) mod country_code;
    pub(super) mod currency;
    pub(crate) mod mime;
    pub(super) mod semiparsed_vec;
    pub(super) mod until_date;
}

pub(crate) mod serde_opt_date_from_unix_timestamp {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub(crate) fn serialize<S>(
        this: &Option<DateTime<Utc>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        this.map(|dt| dt.timestamp()).serialize(serializer)
    }

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Option::<i64>::deserialize(deserializer)?
            .map(|timestamp| DateTime::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc)))
    }

    pub(crate) fn none<T>() -> Option<T> {
        None
    }

    #[test]
    fn test() {
        #[derive(Serialize, Deserialize)]
        struct Struct {
            #[serde(with = "crate::types::serde_opt_date_from_unix_timestamp")]
            #[serde(default = "crate::types::serde_opt_date_from_unix_timestamp::none")]
            date: Option<DateTime<Utc>>,
        }

        {
            let json = r#"{"date":1}"#;
            let expected = DateTime::from_utc(NaiveDateTime::from_timestamp(1, 0), Utc);

            let Struct { date } = serde_json::from_str(json).unwrap();
            assert_eq!(date, Some(expected));
        }

        {
            let json = r#"{}"#;

            let Struct { date } = serde_json::from_str(json).unwrap();
            assert_eq!(date, None);
        }
    }
}

pub(crate) mod serde_date_from_unix_timestamp {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub(crate) fn serialize<S>(this: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        this.timestamp().serialize(serializer)
    }

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let timestamp = i64::deserialize(deserializer)?;

        Ok(DateTime::from_utc(
            NaiveDateTime::from_timestamp(timestamp, 0),
            Utc,
        ))
    }
}
