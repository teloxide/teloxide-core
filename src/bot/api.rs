use url::Url;

use crate::{
    payloads,
    prelude::Requester,
    requests::{JsonRequest, MultipartRequest},
    types::{
        BotCommand, ChatAction, ChatId, ChatPermissions, InlineQueryResult, InputFile, InputMedia,
        InputSticker, LabeledPrice, PassportElementError, TargetMessage,
    },
    Bot,
};

impl Requester for Bot {
    type Err = crate::errors::RequestError;

    type GetUpdates = JsonRequest<payloads::GetUpdates>;

    fn get_updates(&self) -> Self::GetUpdates {
        Self::GetUpdates::new(self.clone(), payloads::GetUpdates::new())
    }

    type SetWebhook = JsonRequest<payloads::SetWebhook>;

    fn set_webhook(&self, url: Url) -> Self::SetWebhook {
        Self::SetWebhook::new(self.clone(), payloads::SetWebhook::new(url))
    }

    type DeleteWebhook = JsonRequest<payloads::DeleteWebhook>;

    fn delete_webhook(&self) -> Self::DeleteWebhook {
        Self::DeleteWebhook::new(self.clone(), payloads::DeleteWebhook::new())
    }

    type GetWebhookInfo = JsonRequest<payloads::GetWebhookInfo>;

    fn get_webhook_info(&self) -> Self::GetWebhookInfo {
        Self::GetWebhookInfo::new(self.clone(), payloads::GetWebhookInfo::new())
    }

    type GetMe = JsonRequest<payloads::GetMe>;

    fn get_me(&self) -> Self::GetMe {
        Self::GetMe::new(self.clone(), payloads::GetMe::new())
    }

    type SendMessage = JsonRequest<payloads::SendMessage>;

    fn send_message(&self, chat_id: ChatId, text: String) -> Self::SendMessage {
        Self::SendMessage::new(self.clone(), payloads::SendMessage::new(chat_id, text))
    }

    type ForwardMessage = JsonRequest<payloads::ForwardMessage>;

    fn forward_message(
        &self,
        chat_id: ChatId,
        from_chat_id: ChatId,
        message_id: i32,
    ) -> Self::ForwardMessage {
        Self::ForwardMessage::new(
            self.clone(),
            payloads::ForwardMessage::new(chat_id, from_chat_id, message_id),
        )
    }

    type SendPhoto = MultipartRequest<payloads::SendPhoto>;

    fn send_photo(&self, chat_id: ChatId, photo: InputFile) -> Self::SendPhoto {
        Self::SendPhoto::new(self.clone(), payloads::SendPhoto::new(chat_id, photo))
    }

    type SendAudio = MultipartRequest<payloads::SendAudio>;

    fn send_audio(&self, chat_id: ChatId, audio: InputFile) -> Self::SendAudio
where {
        Self::SendAudio::new(self.clone(), payloads::SendAudio::new(chat_id, audio))
    }

    type SendDocument = MultipartRequest<payloads::SendDocument>;

    fn send_document(&self, chat_id: ChatId, document: InputFile) -> Self::SendDocument
where {
        Self::SendDocument::new(self.clone(), payloads::SendDocument::new(chat_id, document))
    }

    type SendVideo = MultipartRequest<payloads::SendVideo>;

    fn send_video(&self, chat_id: ChatId, video: InputFile) -> Self::SendVideo
where {
        Self::SendVideo::new(self.clone(), payloads::SendVideo::new(chat_id, video))
    }

    type SendAnimation = MultipartRequest<payloads::SendAnimation>;

    fn send_animation(&self, chat_id: ChatId, animation: InputFile) -> Self::SendAnimation
where {
        Self::SendAnimation::new(
            self.clone(),
            payloads::SendAnimation::new(chat_id, animation),
        )
    }

    type SendVoice = MultipartRequest<payloads::SendVoice>;

    fn send_voice(&self, chat_id: ChatId, voice: InputFile) -> Self::SendVoice
where {
        Self::SendVoice::new(self.clone(), payloads::SendVoice::new(chat_id, voice))
    }

    type SendVideoNote = MultipartRequest<payloads::SendVideoNote>;

    fn send_video_note(&self, chat_id: ChatId, video_note: InputFile) -> Self::SendVideoNote
where {
        Self::SendVideoNote::new(
            self.clone(),
            payloads::SendVideoNote::new(chat_id, video_note),
        )
    }

    type SendMediaGroup = MultipartRequest<payloads::SendMediaGroup>;

    fn send_media_group(&self, chat_id: ChatId, media: Vec<InputMedia>) -> Self::SendMediaGroup {
        Self::SendMediaGroup::new(self.clone(), payloads::SendMediaGroup::new(chat_id, media))
    }

    type SendLocation = JsonRequest<payloads::SendLocation>;

    fn send_location(&self, chat_id: ChatId, latitude: f64, longitude: f64) -> Self::SendLocation
where {
        Self::SendLocation::new(
            self.clone(),
            payloads::SendLocation::new(chat_id, latitude, longitude),
        )
    }

    type EditMessageLiveLocation = JsonRequest<payloads::EditMessageLiveLocation>;

    fn edit_message_live_location(
        &self,
        chat_id: ChatId,
        message_id: i32,
        latitude: f64,
        longitude: f64,
    ) -> Self::EditMessageLiveLocation
where {
        Self::EditMessageLiveLocation::new(
            self.clone(),
            payloads::EditMessageLiveLocation::new(chat_id, message_id, latitude, longitude),
        )
    }

    type EditMessageLiveLocationInline = JsonRequest<payloads::EditMessageLiveLocationInline>;

    fn edit_message_live_location_inline(
        &self,
        inline_message_id: String,
        latitude: f64,
        longitude: f64,
    ) -> Self::EditMessageLiveLocationInline {
        Self::EditMessageLiveLocationInline::new(
            self.clone(),
            payloads::EditMessageLiveLocationInline::new(inline_message_id, latitude, longitude),
        )
    }

    type StopMessageLiveLocation = JsonRequest<payloads::StopMessageLiveLocation>;

    fn stop_message_live_location(
        &self,
        chat_id: ChatId,
        message_id: i32,
        latitude: f64,
        longitude: f64,
    ) -> Self::StopMessageLiveLocation
where {
        Self::StopMessageLiveLocation::new(
            self.clone(),
            payloads::StopMessageLiveLocation::new(chat_id, message_id, latitude, longitude),
        )
    }

    type StopMessageLiveLocationInline = JsonRequest<payloads::StopMessageLiveLocationInline>;

    fn stop_message_live_location_inline(
        &self,
        inline_message_id: String,
        latitude: f64,
        longitude: f64,
    ) -> Self::StopMessageLiveLocationInline {
        Self::StopMessageLiveLocationInline::new(
            self.clone(),
            payloads::StopMessageLiveLocationInline::new(inline_message_id, latitude, longitude),
        )
    }

    type SendVenue = JsonRequest<payloads::SendVenue>;

    fn send_venue(
        &self,
        chat_id: ChatId,
        latitude: f64,
        longitude: f64,
        title: String,
        address: String,
    ) -> Self::SendVenue {
        Self::SendVenue::new(
            self.clone(),
            payloads::SendVenue::new(chat_id, latitude, longitude, title, address),
        )
    }

    type SendContact = JsonRequest<payloads::SendContact>;

    fn send_contact(
        &self,
        chat_id: ChatId,
        phone_number: String,
        first_name: String,
    ) -> Self::SendContact {
        Self::SendContact::new(
            self.clone(),
            payloads::SendContact::new(chat_id, phone_number, first_name),
        )
    }

    type SendPoll = JsonRequest<payloads::SendPoll>;

    fn send_poll(&self, chat_id: ChatId, question: String, options: Vec<String>) -> Self::SendPoll {
        Self::SendPoll::new(
            self.clone(),
            payloads::SendPoll::new(chat_id, question, options),
        )
    }

    type SendDice = JsonRequest<payloads::SendDice>;

    fn send_dice(&self, chat_id: ChatId) -> Self::SendDice
where {
        Self::SendDice::new(self.clone(), payloads::SendDice::new(chat_id))
    }

    type SendChatAction = JsonRequest<payloads::SendChatAction>;

    fn send_chat_action(&self, chat_id: ChatId, action: ChatAction) -> Self::SendChatAction
where {
        Self::SendChatAction::new(self.clone(), payloads::SendChatAction::new(chat_id, action))
    }

    type GetUserProfilePhotos = JsonRequest<payloads::GetUserProfilePhotos>;

    fn get_user_profile_photos(&self, user_id: i64) -> Self::GetUserProfilePhotos {
        Self::GetUserProfilePhotos::new(self.clone(), payloads::GetUserProfilePhotos::new(user_id))
    }

    type GetFile = JsonRequest<payloads::GetFile>;

    fn get_file(&self, file_id: String) -> Self::GetFile {
        Self::GetFile::new(self.clone(), payloads::GetFile::new(file_id))
    }

    type KickChatMember = JsonRequest<payloads::KickChatMember>;

    fn kick_chat_member(&self, chat_id: ChatId, user_id: i64) -> Self::KickChatMember
where {
        Self::KickChatMember::new(
            self.clone(),
            payloads::KickChatMember::new(chat_id, user_id),
        )
    }

    type BanChatMember = JsonRequest<payloads::BanChatMember>;

    fn ban_chat_member(&self, chat_id: ChatId, user_id: i64) -> Self::BanChatMember
where {
        Self::BanChatMember::new(self.clone(), payloads::BanChatMember::new(chat_id, user_id))
    }

    type UnbanChatMember = JsonRequest<payloads::UnbanChatMember>;

    fn unban_chat_member(&self, chat_id: ChatId, user_id: i64) -> Self::UnbanChatMember
where {
        Self::UnbanChatMember::new(
            self.clone(),
            payloads::UnbanChatMember::new(chat_id, user_id),
        )
    }

    type RestrictChatMember = JsonRequest<payloads::RestrictChatMember>;

    fn restrict_chat_member(
        &self,
        chat_id: ChatId,
        user_id: i64,
        permissions: ChatPermissions,
    ) -> Self::RestrictChatMember
where {
        Self::RestrictChatMember::new(
            self.clone(),
            payloads::RestrictChatMember::new(chat_id, user_id, permissions),
        )
    }

    type PromoteChatMember = JsonRequest<payloads::PromoteChatMember>;

    fn promote_chat_member(&self, chat_id: ChatId, user_id: i64) -> Self::PromoteChatMember
where {
        Self::PromoteChatMember::new(
            self.clone(),
            payloads::PromoteChatMember::new(chat_id, user_id),
        )
    }

    type SetChatAdministratorCustomTitle = JsonRequest<payloads::SetChatAdministratorCustomTitle>;

    fn set_chat_administrator_custom_title(
        &self,
        chat_id: ChatId,
        user_id: i64,
        custom_title: String,
    ) -> Self::SetChatAdministratorCustomTitle {
        Self::SetChatAdministratorCustomTitle::new(
            self.clone(),
            payloads::SetChatAdministratorCustomTitle::new(chat_id, user_id, custom_title),
        )
    }

    type BanChatSenderChat = JsonRequest<payloads::BanChatSenderChat>;

    fn ban_chat_sender_chat(
        &self,
        chat_id: ChatId,
        sender_chat_id: i64,
    ) -> Self::BanChatSenderChat
where {
        Self::BanChatSenderChat::new(
            self.clone(),
            payloads::BanChatSenderChat::new(chat_id, sender_chat_id),
        )
    }

    type UnbanChatSenderChat = JsonRequest<payloads::UnbanChatSenderChat>;

    fn unban_chat_sender_chat(
        &self,
        chat_id: ChatId,
        sender_chat_id: i64,
    ) -> Self::UnbanChatSenderChat
where {
        Self::UnbanChatSenderChat::new(
            self.clone(),
            payloads::UnbanChatSenderChat::new(chat_id, sender_chat_id),
        )
    }

    type SetChatPermissions = JsonRequest<payloads::SetChatPermissions>;

    fn set_chat_permissions(
        &self,
        chat_id: ChatId,
        permissions: ChatPermissions,
    ) -> Self::SetChatPermissions
where {
        Self::SetChatPermissions::new(
            self.clone(),
            payloads::SetChatPermissions::new(chat_id, permissions),
        )
    }

    type ExportChatInviteLink = JsonRequest<payloads::ExportChatInviteLink>;

    fn export_chat_invite_link(&self, chat_id: ChatId) -> Self::ExportChatInviteLink
where {
        Self::ExportChatInviteLink::new(self.clone(), payloads::ExportChatInviteLink::new(chat_id))
    }

    type CreateChatInviteLink = JsonRequest<payloads::CreateChatInviteLink>;

    fn create_chat_invite_link(&self, chat_id: ChatId) -> Self::CreateChatInviteLink
where {
        Self::CreateChatInviteLink::new(self.clone(), payloads::CreateChatInviteLink::new(chat_id))
    }

    type EditChatInviteLink = JsonRequest<payloads::EditChatInviteLink>;

    fn edit_chat_invite_link(
        &self,
        chat_id: ChatId,
        invite_link: String,
    ) -> Self::EditChatInviteLink {
        Self::EditChatInviteLink::new(
            self.clone(),
            payloads::EditChatInviteLink::new(chat_id, invite_link),
        )
    }

    type RevokeChatInviteLink = JsonRequest<payloads::RevokeChatInviteLink>;

    fn revoke_chat_invite_link(
        &self,
        chat_id: ChatId,
        invite_link: String,
    ) -> Self::RevokeChatInviteLink {
        Self::RevokeChatInviteLink::new(
            self.clone(),
            payloads::RevokeChatInviteLink::new(chat_id, invite_link),
        )
    }

    type ApproveChatJoinRequest = JsonRequest<payloads::ApproveChatJoinRequest>;

    fn approve_chat_join_request(
        &self,
        chat_id: ChatId,
        user_id: i64,
    ) -> Self::ApproveChatJoinRequest
where {
        Self::ApproveChatJoinRequest::new(
            self.clone(),
            payloads::ApproveChatJoinRequest::new(chat_id, user_id),
        )
    }

    type DeclineChatJoinRequest = JsonRequest<payloads::DeclineChatJoinRequest>;

    fn decline_chat_join_request(
        &self,
        chat_id: ChatId,
        user_id: i64,
    ) -> Self::DeclineChatJoinRequest
where {
        Self::DeclineChatJoinRequest::new(
            self.clone(),
            payloads::DeclineChatJoinRequest::new(chat_id, user_id),
        )
    }

    type SetChatPhoto = MultipartRequest<payloads::SetChatPhoto>;

    fn set_chat_photo(&self, chat_id: ChatId, photo: InputFile) -> Self::SetChatPhoto
where {
        Self::SetChatPhoto::new(self.clone(), payloads::SetChatPhoto::new(chat_id, photo))
    }

    type DeleteChatPhoto = JsonRequest<payloads::DeleteChatPhoto>;

    fn delete_chat_photo(&self, chat_id: ChatId) -> Self::DeleteChatPhoto
where {
        Self::DeleteChatPhoto::new(self.clone(), payloads::DeleteChatPhoto::new(chat_id))
    }

    type SetChatTitle = JsonRequest<payloads::SetChatTitle>;

    fn set_chat_title(&self, chat_id: ChatId, title: String) -> Self::SetChatTitle {
        Self::SetChatTitle::new(self.clone(), payloads::SetChatTitle::new(chat_id, title))
    }

    type SetChatDescription = JsonRequest<payloads::SetChatDescription>;

    fn set_chat_description(&self, chat_id: ChatId) -> Self::SetChatDescription
where {
        Self::SetChatDescription::new(self.clone(), payloads::SetChatDescription::new(chat_id))
    }

    type PinChatMessage = JsonRequest<payloads::PinChatMessage>;

    fn pin_chat_message(&self, chat_id: ChatId, message_id: i32) -> Self::PinChatMessage
where {
        Self::PinChatMessage::new(
            self.clone(),
            payloads::PinChatMessage::new(chat_id, message_id),
        )
    }

    type UnpinChatMessage = JsonRequest<payloads::UnpinChatMessage>;

    fn unpin_chat_message(&self, chat_id: ChatId) -> Self::UnpinChatMessage
where {
        Self::UnpinChatMessage::new(self.clone(), payloads::UnpinChatMessage::new(chat_id))
    }

    type LeaveChat = JsonRequest<payloads::LeaveChat>;

    fn leave_chat(&self, chat_id: ChatId) -> Self::LeaveChat
where {
        Self::LeaveChat::new(self.clone(), payloads::LeaveChat::new(chat_id))
    }

    type GetChat = JsonRequest<payloads::GetChat>;

    fn get_chat(&self, chat_id: ChatId) -> Self::GetChat
where {
        Self::GetChat::new(self.clone(), payloads::GetChat::new(chat_id))
    }

    type GetChatAdministrators = JsonRequest<payloads::GetChatAdministrators>;

    fn get_chat_administrators(&self, chat_id: ChatId) -> Self::GetChatAdministrators
where {
        Self::GetChatAdministrators::new(
            self.clone(),
            payloads::GetChatAdministrators::new(chat_id),
        )
    }

    type GetChatMembersCount = JsonRequest<payloads::GetChatMembersCount>;

    fn get_chat_members_count(&self, chat_id: ChatId) -> Self::GetChatMembersCount
where {
        Self::GetChatMembersCount::new(self.clone(), payloads::GetChatMembersCount::new(chat_id))
    }

    type GetChatMemberCount = JsonRequest<payloads::GetChatMemberCount>;

    fn get_chat_member_count(&self, chat_id: ChatId) -> Self::GetChatMemberCount
where {
        Self::GetChatMemberCount::new(self.clone(), payloads::GetChatMemberCount::new(chat_id))
    }

    type GetChatMember = JsonRequest<payloads::GetChatMember>;

    fn get_chat_member(&self, chat_id: ChatId, user_id: i64) -> Self::GetChatMember
where {
        Self::GetChatMember::new(self.clone(), payloads::GetChatMember::new(chat_id, user_id))
    }

    type SetChatStickerSet = JsonRequest<payloads::SetChatStickerSet>;

    fn set_chat_sticker_set(
        &self,
        chat_id: ChatId,
        sticker_set_name: String,
    ) -> Self::SetChatStickerSet {
        Self::SetChatStickerSet::new(
            self.clone(),
            payloads::SetChatStickerSet::new(chat_id, sticker_set_name),
        )
    }

    type DeleteChatStickerSet = JsonRequest<payloads::DeleteChatStickerSet>;

    fn delete_chat_sticker_set(&self, chat_id: ChatId) -> Self::DeleteChatStickerSet
where {
        Self::DeleteChatStickerSet::new(self.clone(), payloads::DeleteChatStickerSet::new(chat_id))
    }

    type AnswerCallbackQuery = JsonRequest<payloads::AnswerCallbackQuery>;

    fn answer_callback_query(&self, callback_query_id: String) -> Self::AnswerCallbackQuery {
        Self::AnswerCallbackQuery::new(
            self.clone(),
            payloads::AnswerCallbackQuery::new(callback_query_id),
        )
    }

    type SetMyCommands = JsonRequest<payloads::SetMyCommands>;

    fn set_my_commands(&self, commands: Vec<BotCommand>) -> Self::SetMyCommands {
        Self::SetMyCommands::new(self.clone(), payloads::SetMyCommands::new(commands))
    }

    type GetMyCommands = JsonRequest<payloads::GetMyCommands>;

    fn get_my_commands(&self) -> Self::GetMyCommands {
        Self::GetMyCommands::new(self.clone(), payloads::GetMyCommands::new())
    }

    type DeleteMyCommands = JsonRequest<payloads::DeleteMyCommands>;

    fn delete_my_commands(&self) -> Self::DeleteMyCommands {
        Self::DeleteMyCommands::new(self.clone(), payloads::DeleteMyCommands::new())
    }

    type AnswerInlineQuery = JsonRequest<payloads::AnswerInlineQuery>;

    fn answer_inline_query(
        &self,
        inline_query_id: String,
        results: Vec<InlineQueryResult>,
    ) -> Self::AnswerInlineQuery {
        Self::AnswerInlineQuery::new(
            self.clone(),
            payloads::AnswerInlineQuery::new(inline_query_id, results),
        )
    }

    type EditMessageText = JsonRequest<payloads::EditMessageText>;

    fn edit_message_text(
        &self,
        chat_id: ChatId,
        message_id: i32,
        text: String,
    ) -> Self::EditMessageText {
        Self::EditMessageText::new(
            self.clone(),
            payloads::EditMessageText::new(chat_id, message_id, text),
        )
    }

    type EditMessageTextInline = JsonRequest<payloads::EditMessageTextInline>;

    fn edit_message_text_inline(
        &self,
        inline_message_id: String,
        text: String,
    ) -> Self::EditMessageTextInline {
        Self::EditMessageTextInline::new(
            self.clone(),
            payloads::EditMessageTextInline::new(inline_message_id, text),
        )
    }

    type EditMessageCaption = JsonRequest<payloads::EditMessageCaption>;

    fn edit_message_caption(&self, chat_id: ChatId, message_id: i32) -> Self::EditMessageCaption
where {
        Self::EditMessageCaption::new(
            self.clone(),
            payloads::EditMessageCaption::new(chat_id, message_id),
        )
    }

    type EditMessageCaptionInline = JsonRequest<payloads::EditMessageCaptionInline>;

    fn edit_message_caption_inline(
        &self,
        inline_message_id: String,
    ) -> Self::EditMessageCaptionInline {
        Self::EditMessageCaptionInline::new(
            self.clone(),
            payloads::EditMessageCaptionInline::new(inline_message_id),
        )
    }

    type EditMessageMedia = MultipartRequest<payloads::EditMessageMedia>;

    fn edit_message_media(
        &self,
        chat_id: ChatId,
        message_id: i32,
        media: InputMedia,
    ) -> Self::EditMessageMedia
where {
        Self::EditMessageMedia::new(
            self.clone(),
            payloads::EditMessageMedia::new(chat_id, message_id, media),
        )
    }

    type EditMessageMediaInline = MultipartRequest<payloads::EditMessageMediaInline>;

    fn edit_message_media_inline(
        &self,
        inline_message_id: String,
        media: InputMedia,
    ) -> Self::EditMessageMediaInline {
        Self::EditMessageMediaInline::new(
            self.clone(),
            payloads::EditMessageMediaInline::new(inline_message_id, media),
        )
    }

    type EditMessageReplyMarkup = JsonRequest<payloads::EditMessageReplyMarkup>;

    fn edit_message_reply_markup(
        &self,
        chat_id: ChatId,
        message_id: i32,
    ) -> Self::EditMessageReplyMarkup
where {
        Self::EditMessageReplyMarkup::new(
            self.clone(),
            payloads::EditMessageReplyMarkup::new(chat_id, message_id),
        )
    }

    type EditMessageReplyMarkupInline = JsonRequest<payloads::EditMessageReplyMarkupInline>;

    fn edit_message_reply_markup_inline(
        &self,
        inline_message_id: String,
    ) -> Self::EditMessageReplyMarkupInline {
        Self::EditMessageReplyMarkupInline::new(
            self.clone(),
            payloads::EditMessageReplyMarkupInline::new(inline_message_id),
        )
    }

    type StopPoll = JsonRequest<payloads::StopPoll>;

    fn stop_poll(&self, chat_id: ChatId, message_id: i32) -> Self::StopPoll
where {
        Self::StopPoll::new(self.clone(), payloads::StopPoll::new(chat_id, message_id))
    }

    type DeleteMessage = JsonRequest<payloads::DeleteMessage>;

    fn delete_message(&self, chat_id: ChatId, message_id: i32) -> Self::DeleteMessage
where {
        Self::DeleteMessage::new(
            self.clone(),
            payloads::DeleteMessage::new(chat_id, message_id),
        )
    }

    type SendSticker = MultipartRequest<payloads::SendSticker>;

    fn send_sticker(&self, chat_id: ChatId, sticker: InputFile) -> Self::SendSticker
where {
        Self::SendSticker::new(self.clone(), payloads::SendSticker::new(chat_id, sticker))
    }

    type GetStickerSet = JsonRequest<payloads::GetStickerSet>;

    fn get_sticker_set(&self, name: String) -> Self::GetStickerSet {
        Self::GetStickerSet::new(self.clone(), payloads::GetStickerSet::new(name))
    }

    type UploadStickerFile = MultipartRequest<payloads::UploadStickerFile>;

    fn upload_sticker_file(&self, user_id: i64, png_sticker: InputFile) -> Self::UploadStickerFile where
    {
        Self::UploadStickerFile::new(
            self.clone(),
            payloads::UploadStickerFile::new(user_id, png_sticker),
        )
    }

    type CreateNewStickerSet = MultipartRequest<payloads::CreateNewStickerSet>;

    fn create_new_sticker_set(
        &self,
        user_id: i64,
        name: String,
        title: String,
        sticker: InputSticker,
        emojis: String,
    ) -> Self::CreateNewStickerSet {
        Self::CreateNewStickerSet::new(
            self.clone(),
            payloads::CreateNewStickerSet::new(user_id, name, title, sticker, emojis),
        )
    }

    type AddStickerToSet = MultipartRequest<payloads::AddStickerToSet>;

    fn add_sticker_to_set(
        &self,
        user_id: i64,
        name: String,
        sticker: InputSticker,
        emojis: String,
    ) -> Self::AddStickerToSet {
        Self::AddStickerToSet::new(
            self.clone(),
            payloads::AddStickerToSet::new(user_id, name, sticker, emojis),
        )
    }

    type SetStickerPositionInSet = JsonRequest<payloads::SetStickerPositionInSet>;

    fn set_sticker_position_in_set(
        &self,
        sticker: String,
        position: u32,
    ) -> Self::SetStickerPositionInSet {
        Self::SetStickerPositionInSet::new(
            self.clone(),
            payloads::SetStickerPositionInSet::new(sticker, position),
        )
    }

    type DeleteStickerFromSet = JsonRequest<payloads::DeleteStickerFromSet>;

    fn delete_sticker_from_set(&self, sticker: String) -> Self::DeleteStickerFromSet {
        Self::DeleteStickerFromSet::new(self.clone(), payloads::DeleteStickerFromSet::new(sticker))
    }

    type SetStickerSetThumb = MultipartRequest<payloads::SetStickerSetThumb>;

    fn set_sticker_set_thumb(&self, name: String, user_id: i64) -> Self::SetStickerSetThumb {
        Self::SetStickerSetThumb::new(
            self.clone(),
            payloads::SetStickerSetThumb::new(name, user_id),
        )
    }

    type SendInvoice = JsonRequest<payloads::SendInvoice>;

    fn send_invoice(
        &self,
        chat_id: ChatId,
        title: String,
        description: String,
        payload: String,
        provider_token: String,
        currency: String,
        prices: Vec<LabeledPrice>,
    ) -> Self::SendInvoice {
        Self::SendInvoice::new(
            self.clone(),
            payloads::SendInvoice::new(
                chat_id,
                title,
                description,
                payload,
                provider_token,
                currency,
                prices,
            ),
        )
    }

    type AnswerShippingQuery = JsonRequest<payloads::AnswerShippingQuery>;

    fn answer_shipping_query(
        &self,
        shipping_query_id: String,
        ok: bool,
    ) -> Self::AnswerShippingQuery {
        Self::AnswerShippingQuery::new(
            self.clone(),
            payloads::AnswerShippingQuery::new(shipping_query_id, ok),
        )
    }

    type AnswerPreCheckoutQuery = JsonRequest<payloads::AnswerPreCheckoutQuery>;

    fn answer_pre_checkout_query(
        &self,
        pre_checkout_query_id: String,
        ok: bool,
    ) -> Self::AnswerPreCheckoutQuery {
        Self::AnswerPreCheckoutQuery::new(
            self.clone(),
            payloads::AnswerPreCheckoutQuery::new(pre_checkout_query_id, ok),
        )
    }

    type SetPassportDataErrors = JsonRequest<payloads::SetPassportDataErrors>;

    fn set_passport_data_errors(
        &self,
        user_id: i64,
        errors: Vec<PassportElementError>,
    ) -> Self::SetPassportDataErrors {
        Self::SetPassportDataErrors::new(
            self.clone(),
            payloads::SetPassportDataErrors::new(user_id, errors),
        )
    }

    type SendGame = JsonRequest<payloads::SendGame>;

    fn send_game(&self, chat_id: u32, game_short_name: String) -> Self::SendGame {
        Self::SendGame::new(
            self.clone(),
            payloads::SendGame::new(chat_id, game_short_name),
        )
    }

    type SetGameScore = JsonRequest<payloads::SetGameScore>;

    fn set_game_score(
        &self,
        user_id: i64,
        score: u64,
        chat_id: u32,
        message_id: i64,
    ) -> Self::SetGameScore {
        Self::SetGameScore::new(
            self.clone(),
            payloads::SetGameScore::new(user_id, score, chat_id, message_id),
        )
    }

    type SetGameScoreInline = JsonRequest<payloads::SetGameScoreInline>;

    fn set_game_score_inline(
        &self,
        user_id: i64,
        score: u64,
        inline_message_id: String,
    ) -> Self::SetGameScoreInline {
        Self::SetGameScoreInline::new(
            self.clone(),
            payloads::SetGameScoreInline::new(user_id, score, inline_message_id),
        )
    }

    type GetGameHighScores = JsonRequest<payloads::GetGameHighScores>;

    fn get_game_high_scores(&self, user_id: i64, target: TargetMessage) -> Self::GetGameHighScores {
        Self::GetGameHighScores::new(
            self.clone(),
            payloads::GetGameHighScores::new(user_id, target),
        )
    }

    type LogOut = JsonRequest<payloads::LogOut>;

    fn log_out(&self) -> Self::LogOut {
        Self::LogOut::new(self.clone(), payloads::LogOut::new())
    }

    type Close = JsonRequest<payloads::Close>;

    fn close(&self) -> Self::Close {
        Self::Close::new(self.clone(), payloads::Close::new())
    }

    type CopyMessage = JsonRequest<payloads::CopyMessage>;

    fn copy_message(
        &self,
        chat_id: ChatId,
        from_chat_id: ChatId,
        message_id: i32,
    ) -> Self::CopyMessage {
        Self::CopyMessage::new(
            self.clone(),
            payloads::CopyMessage::new(chat_id, from_chat_id, message_id),
        )
    }

    type UnpinAllChatMessages = JsonRequest<payloads::UnpinAllChatMessages>;

    fn unpin_all_chat_messages(&self, chat_id: ChatId) -> Self::UnpinAllChatMessages
where {
        Self::UnpinAllChatMessages::new(self.clone(), payloads::UnpinAllChatMessages::new(chat_id))
    }
}
