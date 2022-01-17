macro_rules! forward_to_unsuported_ty {
    (
        supported: $supported:expr;
        simple { $( $method:ident $arg:ty )* }
        unit { $( $method1:ident $ty:expr )* }
        compound {
            $( $method2:ident $( <$T:ident: ?Sized + Serialize> )? ( $( $args:tt )* ) -> $ret:ty => $message:expr )*
        }
    ) => {
        $(
            fn $method(self, _: $arg) -> Result<Self::Ok, Self::Error> {
                Err(Self::Error::UnsupportedType {
                    ty: stringify!($arg),
                    supported: $supported,
                })
            }
        )+

        $(
            fn $method1(self) -> Result<Self::Ok, Self::Error> {
                Err(Self::Error::UnsupportedType {
                    ty: $ty,
                    supported: $supported,
                })
            }
        )+

        $(
            fn $method2 $( <$T: ?Sized + Serialize> )? (self, $( $args )*) -> Result<$ret, Self::Error> {
                Err(Self::Error::UnsupportedType {
                    ty: $message,
                    supported: $supported,
                })
            }
        )+
    };
}

macro_rules! req_future {
    (
        $v2:vis def: | $( $arg:ident: $ArgTy:ty ),* $(,)? | $body:block

        $(#[$($meta:tt)*])*
        $v:vis $i:ident<$T:ident> ($inner:ident) -> $Out:ty
        $(where $($wh:tt)*)?
    ) => {
        #[pin_project::pin_project]
        pub
        // FIXME(waffle):
        // The `pub` above should ideally be `$v`, but we currently can't do
        // this due to compiler bug, see:
        // - pin_project bug report <https://github.com/taiki-e/pin-project/issues/312>
        // - related rustc issue <https://github.com/rust-lang/rust/issues/81007>
        // - original fix (closed) <https://github.com/rust-lang/rust/pull/81029>
        // - second iteration of the fix <https://github.com/rust-lang/rust/pull/81177>
        struct $i<$T>
        $(where $($wh)*)?
        {
            #[pin]
            inner: $inner::$i<$T>
        }

        impl<$T> $i<$T>
        $(where $($wh)*)?
        {
            $v2 fn new($( $arg: $ArgTy ),*) -> Self {
                Self { inner: $inner::def($( $arg ),*) }
            }
        }

        // HACK(waffle): workaround for https://github.com/rust-lang/rust/issues/55997
        mod $inner {
            #![allow(type_alias_bounds)]

            // Mostly to bring `use`s
            #[allow(unused_imports)]
            use super::{*, $i as _};

            #[cfg(feature = "nightly")]
            pub(crate) type $i<$T>
            $(where $($wh)*)? = impl ::core::future::Future<Output = $Out>;

            #[cfg(feature = "nightly")]
            pub(crate) fn def<$T>($( $arg: $ArgTy ),*) -> $i<$T>
            $(where $($wh)*)?
            {
                $body
            }

            #[cfg(not(feature = "nightly"))]
            pub(crate) type $i<$T>
            $(where $($wh)*)?  = ::core::pin::Pin<Box<dyn ::core::future::Future<Output = $Out> + ::core::marker::Send + 'static>>;

            #[cfg(not(feature = "nightly"))]
            pub(crate) fn def<$T>($( $arg: $ArgTy ),*) -> $i<$T>
            $(where $($wh)*)?
            {
                Box::pin($body)
            }
        }

        impl<$T> ::core::future::Future for $i<$T>
        $(where $($wh)*)?
        {
            type Output = $Out;

            fn poll(self: ::core::pin::Pin<&mut Self>, cx: &mut ::core::task::Context<'_>) -> ::core::task::Poll<Self::Output> {
                let this = self.project();
                this.inner.poll(cx)
            }
        }

    };
}

/// Declares an item with a doc attribute computed by some macro expression.
/// This allows documentation to be dynamically generated based on input.
/// Necessary to work around https://github.com/rust-lang/rust/issues/52607.
macro_rules! calculated_doc {
    (
        $(
            #[doc = $doc:expr]
            $thing:item
        )*
    ) => (
        $(
            #[doc = $doc]
            $thing
        )*
    );
}

/// Declare payload type, implement `Payload` trait and ::new method for it,
/// declare setters trait and implement it for all type which have payload.
macro_rules! impl_payload {
    (
        $(
            @[$multipart_attr:ident]
        )?
        $(
            #[ $($method_meta:tt)* ]
        )*
        $vi:vis $Method:ident ($Setters:ident) => $Ret:ty {
            $(
                required {
                    $(
                        $(
                            #[ $($field_meta:tt)* ]
                        )*
                        $v:vis $fields:ident : $FTy:ty $([$conv:ident])?
                        ,
                    )*
                }
            )?

            $(
                optional {
                    $(
                        $(
                            #[ $($opt_field_meta:tt)* ]
                        )*
                        $opt_v:vis $opt_fields:ident : $OptFTy:ty $([$opt_conv:ident])?
                    ),*
                    $(,)?
                }
            )?
        }
    ) => {
        #[serde_with_macros::skip_serializing_none]
        #[must_use = "Requests do nothing unless sent"]
        $(
            #[ $($method_meta)* ]
        )*
        $vi struct $Method {
            $(
                $(
                    $(
                        #[ $($field_meta)* ]
                    )*
                    $v $fields : $FTy,
                )*
            )?
            $(
                $(
                    $(
                        #[ $($opt_field_meta)* ]
                    )*
                    $opt_v $opt_fields : core::option::Option<$OptFTy>,
                )*
            )?
        }

        impl $Method {
            // We mirror Telegram API and can't do anything with too many arguments.
            #[allow(clippy::too_many_arguments)]
            // It's just easier for macros to generate such code.
            #[allow(clippy::redundant_field_names)]
            // It's obvious what this method does. (If you think it's not, feel free to open a PR)
            #[allow(missing_docs)]
            $vi fn new($($($fields : impl_payload!(@convert? $FTy $([$conv])?)),*)?) -> Self {
                Self {
                    $(
                        $(
                            $fields: impl_payload!(@convert_map ($fields) $([$conv])?),
                        )*
                    )?
                    $(
                        $(
                            $opt_fields: None,
                        )*
                    )?
                }
            }
        }

        impl $crate::requests::Payload for $Method {
            type Output = $Ret;

            const NAME: &'static str = stringify!($Method);
        }

        calculated_doc! {
            #[doc = concat!(
                "Setters for fields of [`",
                stringify!($Method),
                "`]"
            )]
            $vi trait $Setters: $crate::requests::HasPayload<Payload = $Method> + ::core::marker::Sized {
                $(
                    $(
                        impl_payload! { @setter $Method $fields : $FTy $([$conv])? }
                    )*
                )?
                $(
                    $(
                        impl_payload! { @setter_opt $Method $opt_fields : $OptFTy $([$opt_conv])? }
                    )*
                )?
            }
        }

        impl<P> $Setters for P where P: crate::requests::HasPayload<Payload = $Method> {}

        impl_payload! { @[$($multipart_attr)?] $Method req { $($($fields),*)? } opt { $($($opt_fields),*)? } }
    };
    (@setter_opt $Method:ident $field:ident : $FTy:ty [into]) => {
        calculated_doc! {
            #[doc = concat!(
                "Setter for [`",
                stringify!($field),
                "`](",
                stringify!($Method),
                "::",
                stringify!($field),
                ") field."
            )]
            #[allow(clippy::wrong_self_convention)]
            #[must_use = "Payloads and requests do nothing unless sent"]
            fn $field<T>(mut self, value: T) -> Self
            where
                T: Into<$FTy>,
            {
                self.payload_mut().$field = Some(value.into());
                self
            }
        }
    };
    (@setter_opt $Method:ident $field:ident : $FTy:ty [collect]) => {
        calculated_doc! {
            #[doc = concat!(
                "Setter for [`",
                stringify!($field),
                "`](",
                stringify!($Method),
                "::",
                stringify!($field),
                ") field."
            )]
            #[allow(clippy::wrong_self_convention)]
            #[must_use = "Payloads and requests do nothing unless sent"]
            fn $field<T>(mut self, value: T) -> Self
            where
                T: ::core::iter::IntoIterator<Item = <$FTy as ::core::iter::IntoIterator>::Item>,
            {
                self.payload_mut().$field = Some(value.into_iter().collect());
                self
            }
        }
    };
    (@setter_opt $Method:ident $field:ident : $FTy:ty) => {
        calculated_doc! {
            #[doc = concat!(
                "Setter for [`",
                stringify!($field),
                "`](",
                stringify!($Method),
                "::",
                stringify!($field),
                ") field."
            )]
            #[allow(clippy::wrong_self_convention)]
            #[must_use = "Payloads and requests do nothing unless sent"]
            fn $field(mut self, value: $FTy) -> Self {
                self.payload_mut().$field = Some(value);
                self
            }
        }
    };
    (@setter $Method:ident $field:ident : $FTy:ty [into]) => {
        calculated_doc! {
            #[doc = concat!(
                "Setter for [`",
                stringify!($field),
                "`](",
                stringify!($Method),
                "::",
                stringify!($field),
                ") field."
            )]
            #[allow(clippy::wrong_self_convention)]
            #[must_use = "Payloads and requests do nothing unless sent"]
            fn $field<T>(mut self, value: T) -> Self
            where
                T: Into<$FTy>,
            {
                self.payload_mut().$field = value.into();
                self
            }
        }
    };
    (@setter $Method:ident $field:ident : $FTy:ty [collect]) => {
        calculated_doc! {
            #[doc = concat!(
                "Setter for [`",
                stringify!($field),
                "`](",
                stringify!($Method),
                "::",
                stringify!($field),
                ") field."
            )]
            #[allow(clippy::wrong_self_convention)]
            #[must_use = "Payloads and requests do nothing unless sent"]
            fn $field<T>(mut self, value: T) -> Self
            where
                T: ::core::iter::IntoIterator<Item = <$FTy as ::core::iter::IntoIterator>::Item>,
            {
                self.payload_mut().$field = value.into_iter().collect();
                self
            }
        }
    };
    (@setter $Method:ident $field:ident : $FTy:ty) => {
        calculated_doc! {
            #[doc = concat!(
                "Setter for [`",
                stringify!($field),
                "`](",
                stringify!($Method),
                "::",
                stringify!($field),
                ") field."
            )]
            #[allow(clippy::wrong_self_convention)]
            #[must_use = "Payloads and requests do nothing unless sent"]
            fn $field(mut self, value: $FTy) -> Self {
                self.payload_mut().$field = value;
                self
            }
        }
    };
    (@convert? $T:ty [into]) => {
        impl ::core::convert::Into<$T>
    };
    (@convert? $T:ty [collect]) => {
        impl ::core::iter::IntoIterator<Item = <$T as ::core::iter::IntoIterator>::Item>
    };
    (@convert? $T:ty) => {
        $T
    };
    (@convert_map ($e:expr) [into]) => {
        $e.into()
    };
    (@convert_map ($e:expr) [collect]) => {
        $e.into_iter().collect()
    };
    (@convert_map ($e:expr)) => {
        $e
    };
    (@[multipart] $Method:ident req { $($reqf:ident),* } opt { $($optf:ident),*} ) => {
        impl crate::requests::MultipartPayload for $Method {}
        impl crate::requests::multipart_payload::sealed::Sealed for $Method {}
    };
    (@[] $($ignored:tt)*) => {}
}

macro_rules! download_forward {
    ($l:lifetime $T:ident $S:ty {$this:ident => $inner:expr}) => {
        impl<$l, $T: $crate::net::Download<$l>> $crate::net::Download<$l> for $S {
            type Err = <$T as $crate::net::Download<$l>>::Err;

            type Fut = <$T as $crate::net::Download<$l>>::Fut;

            fn download_file(
                &self,
                path: &str,
                destination: &'w mut (dyn tokio::io::AsyncWrite
                             + core::marker::Unpin
                             + core::marker::Send),
            ) -> Self::Fut {
                let $this = self;
                ($inner).download_file(path, destination)
            }

            type StreamErr = <$T as $crate::net::Download<$l>>::StreamErr;

            type Stream = <$T as $crate::net::Download<$l>>::Stream;

            fn download_file_stream(&self, path: &str) -> Self::Stream {
                let $this = self;
                ($inner).download_file_stream(path)
            }
        }
    };
}

// This macro is auto generated by [`cg`] from [`schema`].
//
// **DO NOT EDIT THIS MACRO**,
//
// Edit `cg` or `schema` instead.
//
// [cg]: https://github.com/teloxide/cg
// [`schema`]: https://github.com/WaffleLapkin/tg-methods-schema
macro_rules! requester_forward {
    ($i:ident $(, $rest:ident )* $(,)? => $body:ident, $ty:ident ) => {
        requester_forward!(@method $i $body $ty);
        $(
            requester_forward!(@method $rest $body $ty);
        )*
    };

    (@method get_updates $body:ident $ty:ident) => {
        type GetUpdates = $ty![GetUpdates];

        fn get_updates(&self) -> Self::GetUpdates {
            let this = self;
            $body!(get_updates this ())
        }
    };
    (@method set_webhook $body:ident $ty:ident) => {
        type SetWebhook = $ty![SetWebhook];

        fn set_webhook(&self, url: Url) -> Self::SetWebhook {
            let this = self;
            $body!(set_webhook this (url: Url))
        }
    };
    (@method delete_webhook $body:ident $ty:ident) => {
        type DeleteWebhook = $ty![DeleteWebhook];

        fn delete_webhook(&self) -> Self::DeleteWebhook {
            let this = self;
            $body!(delete_webhook this ())
        }
    };
    (@method get_webhook_info $body:ident $ty:ident) => {
        type GetWebhookInfo = $ty![GetWebhookInfo];

        fn get_webhook_info(&self) -> Self::GetWebhookInfo {
            let this = self;
            $body!(get_webhook_info this ())
        }
    };
    (@method get_me $body:ident $ty:ident) => {
        type GetMe = $ty![GetMe];

        fn get_me(&self) -> Self::GetMe {
            let this = self;
            $body!(get_me this ())
        }
    };
    (@method log_out $body:ident $ty:ident) => {
        type LogOut = $ty![LogOut];

        fn log_out(&self) -> Self::LogOut {
            let this = self;
            $body!(log_out this ())
        }
    };
    (@method close $body:ident $ty:ident) => {
        type Close = $ty![Close];

        fn close(&self) -> Self::Close {
            let this = self;
            $body!(close this ())
        }
    };
    (@method send_message $body:ident $ty:ident) => {
        type SendMessage = $ty![SendMessage];

        fn send_message(&self, chat_id: ChatId, text: String) -> Self::SendMessage {
            let this = self;
            $body!(send_message this (chat_id: ChatId, text: String))
        }
    };
    (@method forward_message $body:ident $ty:ident) => {
        type ForwardMessage = $ty![ForwardMessage];

        fn forward_message(&self, chat_id: ChatId, from_chat_id: ChatId, message_id: i32) -> Self::ForwardMessage {
            let this = self;
            $body!(forward_message this (chat_id: ChatId, from_chat_id: ChatId, message_id: i32))
        }
    };
    (@method copy_message $body:ident $ty:ident) => {
        type CopyMessage = $ty![CopyMessage];

        fn copy_message(&self, chat_id: ChatId, from_chat_id: ChatId, message_id: i32) -> Self::CopyMessage {
            let this = self;
            $body!(copy_message this (chat_id: ChatId, from_chat_id: ChatId, message_id: i32))
        }
    };
    (@method send_photo $body:ident $ty:ident) => {
        type SendPhoto = $ty![SendPhoto];

        fn send_photo(&self, chat_id: ChatId, photo: InputFile) -> Self::SendPhoto {
            let this = self;
            $body!(send_photo this (chat_id: ChatId, photo: InputFile))
        }
    };
    (@method send_audio $body:ident $ty:ident) => {
        type SendAudio = $ty![SendAudio];

        fn send_audio(&self, chat_id: ChatId, audio: InputFile) -> Self::SendAudio {
            let this = self;
            $body!(send_audio this (chat_id: ChatId, audio: InputFile))
        }
    };
    (@method send_document $body:ident $ty:ident) => {
        type SendDocument = $ty![SendDocument];

        fn send_document(&self, chat_id: ChatId, document: InputFile) -> Self::SendDocument {
            let this = self;
            $body!(send_document this (chat_id: ChatId, document: InputFile))
        }
    };
    (@method send_video $body:ident $ty:ident) => {
        type SendVideo = $ty![SendVideo];

        fn send_video(&self, chat_id: ChatId, video: InputFile) -> Self::SendVideo {
            let this = self;
            $body!(send_video this (chat_id: ChatId, video: InputFile))
        }
    };
    (@method send_animation $body:ident $ty:ident) => {
        type SendAnimation = $ty![SendAnimation];

        fn send_animation(&self, chat_id: ChatId, animation: InputFile) -> Self::SendAnimation {
            let this = self;
            $body!(send_animation this (chat_id: ChatId, animation: InputFile))
        }
    };
    (@method send_voice $body:ident $ty:ident) => {
        type SendVoice = $ty![SendVoice];

        fn send_voice(&self, chat_id: ChatId, voice: InputFile) -> Self::SendVoice {
            let this = self;
            $body!(send_voice this (chat_id: ChatId, voice: InputFile))
        }
    };
    (@method send_video_note $body:ident $ty:ident) => {
        type SendVideoNote = $ty![SendVideoNote];

        fn send_video_note(&self, chat_id: ChatId, video_note: InputFile) -> Self::SendVideoNote {
            let this = self;
            $body!(send_video_note this (chat_id: ChatId, video_note: InputFile))
        }
    };
    (@method send_media_group $body:ident $ty:ident) => {
        type SendMediaGroup = $ty![SendMediaGroup];

        fn send_media_group(&self, chat_id: ChatId, media: Vec<InputMedia>) -> Self::SendMediaGroup {
            let this = self;
            $body!(send_media_group this (chat_id: ChatId, media: Vec<InputMedia>))
        }
    };
    (@method send_location $body:ident $ty:ident) => {
        type SendLocation = $ty![SendLocation];

        fn send_location(&self, chat_id: ChatId, latitude: f64, longitude: f64) -> Self::SendLocation {
            let this = self;
            $body!(send_location this (chat_id: ChatId, latitude: f64, longitude: f64))
        }
    };
    (@method edit_message_live_location $body:ident $ty:ident) => {
        type EditMessageLiveLocation = $ty![EditMessageLiveLocation];

        fn edit_message_live_location(&self, chat_id: ChatId, message_id: i32, latitude: f64, longitude: f64) -> Self::EditMessageLiveLocation {
            let this = self;
            $body!(edit_message_live_location this (chat_id: ChatId, message_id: i32, latitude: f64, longitude: f64))
        }
    };
    (@method edit_message_live_location_inline $body:ident $ty:ident) => {
        type EditMessageLiveLocationInline = $ty![EditMessageLiveLocationInline];

        fn edit_message_live_location_inline(&self, inline_message_id: String, latitude: f64, longitude: f64) -> Self::EditMessageLiveLocationInline {
            let this = self;
            $body!(edit_message_live_location_inline this (inline_message_id: String, latitude: f64, longitude: f64))
        }
    };
    (@method stop_message_live_location $body:ident $ty:ident) => {
        type StopMessageLiveLocation = $ty![StopMessageLiveLocation];

        fn stop_message_live_location(&self, chat_id: ChatId, message_id: i32, latitude: f64, longitude: f64) -> Self::StopMessageLiveLocation {
            let this = self;
            $body!(stop_message_live_location this (chat_id: ChatId, message_id: i32, latitude: f64, longitude: f64))
        }
    };
    (@method stop_message_live_location_inline $body:ident $ty:ident) => {
        type StopMessageLiveLocationInline = $ty![StopMessageLiveLocationInline];

        fn stop_message_live_location_inline(&self, inline_message_id: String, latitude: f64, longitude: f64) -> Self::StopMessageLiveLocationInline {
            let this = self;
            $body!(stop_message_live_location_inline this (inline_message_id: String, latitude: f64, longitude: f64))
        }
    };
    (@method send_venue $body:ident $ty:ident) => {
        type SendVenue = $ty![SendVenue];

        fn send_venue(&self, chat_id: ChatId, latitude: f64, longitude: f64, title: String, address: String) -> Self::SendVenue {
            let this = self;
            $body!(send_venue this (chat_id: ChatId, latitude: f64, longitude: f64, title: String, address: String))
        }
    };
    (@method send_contact $body:ident $ty:ident) => {
        type SendContact = $ty![SendContact];

        fn send_contact(&self, chat_id: ChatId, phone_number: String, first_name: String) -> Self::SendContact {
            let this = self;
            $body!(send_contact this (chat_id: ChatId, phone_number: String, first_name: String))
        }
    };
    (@method send_poll $body:ident $ty:ident) => {
        type SendPoll = $ty![SendPoll];

        fn send_poll(&self, chat_id: ChatId, question: String, options: Vec<String>) -> Self::SendPoll {
            let this = self;
            $body!(send_poll this (chat_id: ChatId, question: String, options: Vec<String>))
        }
    };
    (@method send_dice $body:ident $ty:ident) => {
        type SendDice = $ty![SendDice];

        fn send_dice(&self, chat_id: ChatId) -> Self::SendDice {
            let this = self;
            $body!(send_dice this (chat_id: ChatId))
        }
    };
    (@method send_chat_action $body:ident $ty:ident) => {
        type SendChatAction = $ty![SendChatAction];

        fn send_chat_action(&self, chat_id: ChatId, action: ChatAction) -> Self::SendChatAction {
            let this = self;
            $body!(send_chat_action this (chat_id: ChatId, action: ChatAction))
        }
    };
    (@method get_user_profile_photos $body:ident $ty:ident) => {
        type GetUserProfilePhotos = $ty![GetUserProfilePhotos];

        fn get_user_profile_photos(&self, user_id: i64) -> Self::GetUserProfilePhotos {
            let this = self;
            $body!(get_user_profile_photos this (user_id: i64))
        }
    };
    (@method get_file $body:ident $ty:ident) => {
        type GetFile = $ty![GetFile];

        fn get_file(&self, file_id: String) -> Self::GetFile {
            let this = self;
            $body!(get_file this (file_id: String))
        }
    };
    (@method ban_chat_member $body:ident $ty:ident) => {
        type BanChatMember = $ty![BanChatMember];

        fn ban_chat_member(&self, chat_id: ChatId, user_id: i64) -> Self::BanChatMember {
            let this = self;
            $body!(ban_chat_member this (chat_id: ChatId, user_id: i64))
        }
    };
    (@method kick_chat_member $body:ident $ty:ident) => {
        type KickChatMember = $ty![KickChatMember];

        fn kick_chat_member(&self, chat_id: ChatId, user_id: i64) -> Self::KickChatMember {
            let this = self;
            $body!(kick_chat_member this (chat_id: ChatId, user_id: i64))
        }
    };
    (@method unban_chat_member $body:ident $ty:ident) => {
        type UnbanChatMember = $ty![UnbanChatMember];

        fn unban_chat_member(&self, chat_id: ChatId, user_id: i64) -> Self::UnbanChatMember {
            let this = self;
            $body!(unban_chat_member this (chat_id: ChatId, user_id: i64))
        }
    };
    (@method restrict_chat_member $body:ident $ty:ident) => {
        type RestrictChatMember = $ty![RestrictChatMember];

        fn restrict_chat_member(&self, chat_id: ChatId, user_id: i64, permissions: ChatPermissions) -> Self::RestrictChatMember {
            let this = self;
            $body!(restrict_chat_member this (chat_id: ChatId, user_id: i64, permissions: ChatPermissions))
        }
    };
    (@method promote_chat_member $body:ident $ty:ident) => {
        type PromoteChatMember = $ty![PromoteChatMember];

        fn promote_chat_member(&self, chat_id: ChatId, user_id: i64) -> Self::PromoteChatMember {
            let this = self;
            $body!(promote_chat_member this (chat_id: ChatId, user_id: i64))
        }
    };
    (@method set_chat_administrator_custom_title $body:ident $ty:ident) => {
        type SetChatAdministratorCustomTitle = $ty![SetChatAdministratorCustomTitle];

        fn set_chat_administrator_custom_title(&self, chat_id: ChatId, user_id: i64, custom_title: String) -> Self::SetChatAdministratorCustomTitle {
            let this = self;
            $body!(set_chat_administrator_custom_title this (chat_id: ChatId, user_id: i64, custom_title: String))
        }
    };
    (@method ban_chat_sender_chat $body:ident $ty:ident) => {
        type BanChatSenderChat = $ty![BanChatSenderChat];

        fn ban_chat_sender_chat(&self, chat_id: ChatId, sender_chat_id: i64) -> Self::BanChatSenderChat {
            let this = self;
            $body!(ban_chat_sender_chat this (chat_id: ChatId, sender_chat_id: i64))
        }
    };
    (@method unban_chat_sender_chat $body:ident $ty:ident) => {
        type UnbanChatSenderChat = $ty![UnbanChatSenderChat];

        fn unban_chat_sender_chat(&self, chat_id: ChatId, sender_chat_id: i64) -> Self::UnbanChatSenderChat {
            let this = self;
            $body!(unban_chat_sender_chat this (chat_id: ChatId, sender_chat_id: i64))
        }
    };
    (@method set_chat_permissions $body:ident $ty:ident) => {
        type SetChatPermissions = $ty![SetChatPermissions];

        fn set_chat_permissions(&self, chat_id: ChatId, permissions: ChatPermissions) -> Self::SetChatPermissions {
            let this = self;
            $body!(set_chat_permissions this (chat_id: ChatId, permissions: ChatPermissions))
        }
    };
    (@method export_chat_invite_link $body:ident $ty:ident) => {
        type ExportChatInviteLink = $ty![ExportChatInviteLink];

        fn export_chat_invite_link(&self, chat_id: ChatId) -> Self::ExportChatInviteLink {
            let this = self;
            $body!(export_chat_invite_link this (chat_id: ChatId))
        }
    };
    (@method create_chat_invite_link $body:ident $ty:ident) => {
        type CreateChatInviteLink = $ty![CreateChatInviteLink];

        fn create_chat_invite_link(&self, chat_id: ChatId) -> Self::CreateChatInviteLink {
            let this = self;
            $body!(create_chat_invite_link this (chat_id: ChatId))
        }
    };
    (@method edit_chat_invite_link $body:ident $ty:ident) => {
        type EditChatInviteLink = $ty![EditChatInviteLink];

        fn edit_chat_invite_link(&self, chat_id: ChatId, invite_link: String) -> Self::EditChatInviteLink {
            let this = self;
            $body!(edit_chat_invite_link this (chat_id: ChatId, invite_link: String))
        }
    };
    (@method revoke_chat_invite_link $body:ident $ty:ident) => {
        type RevokeChatInviteLink = $ty![RevokeChatInviteLink];

        fn revoke_chat_invite_link(&self, chat_id: ChatId, invite_link: String) -> Self::RevokeChatInviteLink {
            let this = self;
            $body!(revoke_chat_invite_link this (chat_id: ChatId, invite_link: String))
        }
    };
    (@method approve_chat_join_request $body:ident $ty:ident) => {
        type ApproveChatJoinRequest = $ty![ApproveChatJoinRequest];

        fn approve_chat_join_request(&self, chat_id: ChatId, user_id: i64) -> Self::ApproveChatJoinRequest {
            let this = self;
            $body!(approve_chat_join_request this (chat_id: ChatId, user_id: i64))
        }
    };
    (@method decline_chat_join_request $body:ident $ty:ident) => {
        type DeclineChatJoinRequest = $ty![DeclineChatJoinRequest];

        fn decline_chat_join_request(&self, chat_id: ChatId, user_id: i64) -> Self::DeclineChatJoinRequest {
            let this = self;
            $body!(decline_chat_join_request this (chat_id: ChatId, user_id: i64))
        }
    };
    (@method set_chat_photo $body:ident $ty:ident) => {
        type SetChatPhoto = $ty![SetChatPhoto];

        fn set_chat_photo(&self, chat_id: ChatId, photo: InputFile) -> Self::SetChatPhoto {
            let this = self;
            $body!(set_chat_photo this (chat_id: ChatId, photo: InputFile))
        }
    };
    (@method delete_chat_photo $body:ident $ty:ident) => {
        type DeleteChatPhoto = $ty![DeleteChatPhoto];

        fn delete_chat_photo(&self, chat_id: ChatId) -> Self::DeleteChatPhoto {
            let this = self;
            $body!(delete_chat_photo this (chat_id: ChatId))
        }
    };
    (@method set_chat_title $body:ident $ty:ident) => {
        type SetChatTitle = $ty![SetChatTitle];

        fn set_chat_title(&self, chat_id: ChatId, title: String) -> Self::SetChatTitle {
            let this = self;
            $body!(set_chat_title this (chat_id: ChatId, title: String))
        }
    };
    (@method set_chat_description $body:ident $ty:ident) => {
        type SetChatDescription = $ty![SetChatDescription];

        fn set_chat_description(&self, chat_id: ChatId) -> Self::SetChatDescription {
            let this = self;
            $body!(set_chat_description this (chat_id: ChatId))
        }
    };
    (@method pin_chat_message $body:ident $ty:ident) => {
        type PinChatMessage = $ty![PinChatMessage];

        fn pin_chat_message(&self, chat_id: ChatId, message_id: i32) -> Self::PinChatMessage {
            let this = self;
            $body!(pin_chat_message this (chat_id: ChatId, message_id: i32))
        }
    };
    (@method unpin_chat_message $body:ident $ty:ident) => {
        type UnpinChatMessage = $ty![UnpinChatMessage];

        fn unpin_chat_message(&self, chat_id: ChatId) -> Self::UnpinChatMessage {
            let this = self;
            $body!(unpin_chat_message this (chat_id: ChatId))
        }
    };
    (@method unpin_all_chat_messages $body:ident $ty:ident) => {
        type UnpinAllChatMessages = $ty![UnpinAllChatMessages];

        fn unpin_all_chat_messages(&self, chat_id: ChatId) -> Self::UnpinAllChatMessages {
            let this = self;
            $body!(unpin_all_chat_messages this (chat_id: ChatId))
        }
    };
    (@method leave_chat $body:ident $ty:ident) => {
        type LeaveChat = $ty![LeaveChat];

        fn leave_chat(&self, chat_id: ChatId) -> Self::LeaveChat {
            let this = self;
            $body!(leave_chat this (chat_id: ChatId))
        }
    };
    (@method get_chat $body:ident $ty:ident) => {
        type GetChat = $ty![GetChat];

        fn get_chat(&self, chat_id: ChatId) -> Self::GetChat {
            let this = self;
            $body!(get_chat this (chat_id: ChatId))
        }
    };
    (@method get_chat_administrators $body:ident $ty:ident) => {
        type GetChatAdministrators = $ty![GetChatAdministrators];

        fn get_chat_administrators(&self, chat_id: ChatId) -> Self::GetChatAdministrators {
            let this = self;
            $body!(get_chat_administrators this (chat_id: ChatId))
        }
    };
    (@method get_chat_member_count $body:ident $ty:ident) => {
        type GetChatMemberCount = $ty![GetChatMemberCount];

        fn get_chat_member_count(&self, chat_id: ChatId) -> Self::GetChatMemberCount {
            let this = self;
            $body!(get_chat_member_count this (chat_id: ChatId))
        }
    };
    (@method get_chat_members_count $body:ident $ty:ident) => {
        type GetChatMembersCount = $ty![GetChatMembersCount];

        fn get_chat_members_count(&self, chat_id: ChatId) -> Self::GetChatMembersCount {
            let this = self;
            $body!(get_chat_members_count this (chat_id: ChatId))
        }
    };
    (@method get_chat_member $body:ident $ty:ident) => {
        type GetChatMember = $ty![GetChatMember];

        fn get_chat_member(&self, chat_id: ChatId, user_id: i64) -> Self::GetChatMember {
            let this = self;
            $body!(get_chat_member this (chat_id: ChatId, user_id: i64))
        }
    };
    (@method set_chat_sticker_set $body:ident $ty:ident) => {
        type SetChatStickerSet = $ty![SetChatStickerSet];

        fn set_chat_sticker_set(&self, chat_id: ChatId, sticker_set_name: String) -> Self::SetChatStickerSet {
            let this = self;
            $body!(set_chat_sticker_set this (chat_id: ChatId, sticker_set_name: String))
        }
    };
    (@method delete_chat_sticker_set $body:ident $ty:ident) => {
        type DeleteChatStickerSet = $ty![DeleteChatStickerSet];

        fn delete_chat_sticker_set(&self, chat_id: ChatId) -> Self::DeleteChatStickerSet {
            let this = self;
            $body!(delete_chat_sticker_set this (chat_id: ChatId))
        }
    };
    (@method answer_callback_query $body:ident $ty:ident) => {
        type AnswerCallbackQuery = $ty![AnswerCallbackQuery];

        fn answer_callback_query(&self, callback_query_id: String) -> Self::AnswerCallbackQuery {
            let this = self;
            $body!(answer_callback_query this (callback_query_id: String))
        }
    };
    (@method set_my_commands $body:ident $ty:ident) => {
        type SetMyCommands = $ty![SetMyCommands];

        fn set_my_commands(&self, commands: Vec<BotCommand>) -> Self::SetMyCommands {
            let this = self;
            $body!(set_my_commands this (commands: Vec<BotCommand>))
        }
    };
    (@method get_my_commands $body:ident $ty:ident) => {
        type GetMyCommands = $ty![GetMyCommands];

        fn get_my_commands(&self) -> Self::GetMyCommands {
            let this = self;
            $body!(get_my_commands this ())
        }
    };
    (@method delete_my_commands $body:ident $ty:ident) => {
        type DeleteMyCommands = $ty![DeleteMyCommands];

        fn delete_my_commands(&self) -> Self::DeleteMyCommands {
            let this = self;
            $body!(delete_my_commands this ())
        }
    };
    (@method answer_inline_query $body:ident $ty:ident) => {
        type AnswerInlineQuery = $ty![AnswerInlineQuery];

        fn answer_inline_query(&self, inline_query_id: String, results: Vec<InlineQueryResult>) -> Self::AnswerInlineQuery {
            let this = self;
            $body!(answer_inline_query this (inline_query_id: String, results: Vec<InlineQueryResult>))
        }
    };
    (@method edit_message_text $body:ident $ty:ident) => {
        type EditMessageText = $ty![EditMessageText];

        fn edit_message_text(&self, chat_id: ChatId, message_id: i32, text: String) -> Self::EditMessageText {
            let this = self;
            $body!(edit_message_text this (chat_id: ChatId, message_id: i32, text: String))
        }
    };
    (@method edit_message_text_inline $body:ident $ty:ident) => {
        type EditMessageTextInline = $ty![EditMessageTextInline];

        fn edit_message_text_inline(&self, inline_message_id: String, text: String) -> Self::EditMessageTextInline {
            let this = self;
            $body!(edit_message_text_inline this (inline_message_id: String, text: String))
        }
    };
    (@method edit_message_caption $body:ident $ty:ident) => {
        type EditMessageCaption = $ty![EditMessageCaption];

        fn edit_message_caption(&self, chat_id: ChatId, message_id: i32) -> Self::EditMessageCaption {
            let this = self;
            $body!(edit_message_caption this (chat_id: ChatId, message_id: i32))
        }
    };
    (@method edit_message_caption_inline $body:ident $ty:ident) => {
        type EditMessageCaptionInline = $ty![EditMessageCaptionInline];

        fn edit_message_caption_inline(&self, inline_message_id: String) -> Self::EditMessageCaptionInline {
            let this = self;
            $body!(edit_message_caption_inline this (inline_message_id: String))
        }
    };
    (@method edit_message_media $body:ident $ty:ident) => {
        type EditMessageMedia = $ty![EditMessageMedia];

        fn edit_message_media(&self, chat_id: ChatId, message_id: i32, media: InputMedia) -> Self::EditMessageMedia {
            let this = self;
            $body!(edit_message_media this (chat_id: ChatId, message_id: i32, media: InputMedia))
        }
    };
    (@method edit_message_media_inline $body:ident $ty:ident) => {
        type EditMessageMediaInline = $ty![EditMessageMediaInline];

        fn edit_message_media_inline(&self, inline_message_id: String, media: InputMedia) -> Self::EditMessageMediaInline {
            let this = self;
            $body!(edit_message_media_inline this (inline_message_id: String, media: InputMedia))
        }
    };
    (@method edit_message_reply_markup $body:ident $ty:ident) => {
        type EditMessageReplyMarkup = $ty![EditMessageReplyMarkup];

        fn edit_message_reply_markup(&self, chat_id: ChatId, message_id: i32) -> Self::EditMessageReplyMarkup {
            let this = self;
            $body!(edit_message_reply_markup this (chat_id: ChatId, message_id: i32))
        }
    };
    (@method edit_message_reply_markup_inline $body:ident $ty:ident) => {
        type EditMessageReplyMarkupInline = $ty![EditMessageReplyMarkupInline];

        fn edit_message_reply_markup_inline(&self, inline_message_id: String) -> Self::EditMessageReplyMarkupInline {
            let this = self;
            $body!(edit_message_reply_markup_inline this (inline_message_id: String))
        }
    };
    (@method stop_poll $body:ident $ty:ident) => {
        type StopPoll = $ty![StopPoll];

        fn stop_poll(&self, chat_id: ChatId, message_id: i32) -> Self::StopPoll {
            let this = self;
            $body!(stop_poll this (chat_id: ChatId, message_id: i32))
        }
    };
    (@method delete_message $body:ident $ty:ident) => {
        type DeleteMessage = $ty![DeleteMessage];

        fn delete_message(&self, chat_id: ChatId, message_id: i32) -> Self::DeleteMessage {
            let this = self;
            $body!(delete_message this (chat_id: ChatId, message_id: i32))
        }
    };
    (@method send_sticker $body:ident $ty:ident) => {
        type SendSticker = $ty![SendSticker];

        fn send_sticker(&self, chat_id: ChatId, sticker: InputFile) -> Self::SendSticker {
            let this = self;
            $body!(send_sticker this (chat_id: ChatId, sticker: InputFile))
        }
    };
    (@method get_sticker_set $body:ident $ty:ident) => {
        type GetStickerSet = $ty![GetStickerSet];

        fn get_sticker_set(&self, name: String) -> Self::GetStickerSet {
            let this = self;
            $body!(get_sticker_set this (name: String))
        }
    };
    (@method upload_sticker_file $body:ident $ty:ident) => {
        type UploadStickerFile = $ty![UploadStickerFile];

        fn upload_sticker_file(&self, user_id: i64, png_sticker: InputFile) -> Self::UploadStickerFile {
            let this = self;
            $body!(upload_sticker_file this (user_id: i64, png_sticker: InputFile))
        }
    };
    (@method create_new_sticker_set $body:ident $ty:ident) => {
        type CreateNewStickerSet = $ty![CreateNewStickerSet];

        fn create_new_sticker_set(&self, user_id: i64, name: String, title: String, sticker: InputSticker, emojis: String) -> Self::CreateNewStickerSet {
            let this = self;
            $body!(create_new_sticker_set this (user_id: i64, name: String, title: String, sticker: InputSticker, emojis: String))
        }
    };
    (@method add_sticker_to_set $body:ident $ty:ident) => {
        type AddStickerToSet = $ty![AddStickerToSet];

        fn add_sticker_to_set(&self, user_id: i64, name: String, sticker: InputSticker, emojis: String) -> Self::AddStickerToSet {
            let this = self;
            $body!(add_sticker_to_set this (user_id: i64, name: String, sticker: InputSticker, emojis: String))
        }
    };
    (@method set_sticker_position_in_set $body:ident $ty:ident) => {
        type SetStickerPositionInSet = $ty![SetStickerPositionInSet];

        fn set_sticker_position_in_set(&self, sticker: String, position: u32) -> Self::SetStickerPositionInSet {
            let this = self;
            $body!(set_sticker_position_in_set this (sticker: String, position: u32))
        }
    };
    (@method delete_sticker_from_set $body:ident $ty:ident) => {
        type DeleteStickerFromSet = $ty![DeleteStickerFromSet];

        fn delete_sticker_from_set(&self, sticker: String) -> Self::DeleteStickerFromSet {
            let this = self;
            $body!(delete_sticker_from_set this (sticker: String))
        }
    };
    (@method set_sticker_set_thumb $body:ident $ty:ident) => {
        type SetStickerSetThumb = $ty![SetStickerSetThumb];

        fn set_sticker_set_thumb(&self, name: String, user_id: i64) -> Self::SetStickerSetThumb {
            let this = self;
            $body!(set_sticker_set_thumb this (name: String, user_id: i64))
        }
    };
    (@method send_invoice $body:ident $ty:ident) => {
        type SendInvoice = $ty![SendInvoice];

        fn send_invoice(&self, chat_id: ChatId, title: String, description: String, payload: String, provider_token: String, currency: String, prices: Vec<LabeledPrice>) -> Self::SendInvoice {
            let this = self;
            $body!(send_invoice this (chat_id: ChatId, title: String, description: String, payload: String, provider_token: String, currency: String, prices: Vec<LabeledPrice>))
        }
    };
    (@method answer_shipping_query $body:ident $ty:ident) => {
        type AnswerShippingQuery = $ty![AnswerShippingQuery];

        fn answer_shipping_query(&self, shipping_query_id: String, ok: bool) -> Self::AnswerShippingQuery {
            let this = self;
            $body!(answer_shipping_query this (shipping_query_id: String, ok: bool))
        }
    };
    (@method answer_pre_checkout_query $body:ident $ty:ident) => {
        type AnswerPreCheckoutQuery = $ty![AnswerPreCheckoutQuery];

        fn answer_pre_checkout_query(&self, pre_checkout_query_id: String, ok: bool) -> Self::AnswerPreCheckoutQuery {
            let this = self;
            $body!(answer_pre_checkout_query this (pre_checkout_query_id: String, ok: bool))
        }
    };
    (@method set_passport_data_errors $body:ident $ty:ident) => {
        type SetPassportDataErrors = $ty![SetPassportDataErrors];

        fn set_passport_data_errors(&self, user_id: i64, errors: Vec<PassportElementError>) -> Self::SetPassportDataErrors {
            let this = self;
            $body!(set_passport_data_errors this (user_id: i64, errors: Vec<PassportElementError>))
        }
    };
    (@method send_game $body:ident $ty:ident) => {
        type SendGame = $ty![SendGame];

        fn send_game(&self, chat_id: u32, game_short_name: String) -> Self::SendGame {
            let this = self;
            $body!(send_game this (chat_id: u32, game_short_name: String))
        }
    };
    (@method set_game_score $body:ident $ty:ident) => {
        type SetGameScore = $ty![SetGameScore];

        fn set_game_score(&self, user_id: i64, score: u64, chat_id: u32, message_id: i64) -> Self::SetGameScore {
            let this = self;
            $body!(set_game_score this (user_id: i64, score: u64, chat_id: u32, message_id: i64))
        }
    };
    (@method set_game_score_inline $body:ident $ty:ident) => {
        type SetGameScoreInline = $ty![SetGameScoreInline];

        fn set_game_score_inline(&self, user_id: i64, score: u64, inline_message_id: String) -> Self::SetGameScoreInline {
            let this = self;
            $body!(set_game_score_inline this (user_id: i64, score: u64, inline_message_id: String))
        }
    };
    (@method get_game_high_scores $body:ident $ty:ident) => {
        type GetGameHighScores = $ty![GetGameHighScores];

        fn get_game_high_scores(&self, user_id: i64, target: TargetMessage) -> Self::GetGameHighScores {
            let this = self;
            $body!(get_game_high_scores this (user_id: i64, target: TargetMessage))
        }
    };
}
