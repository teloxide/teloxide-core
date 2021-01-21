use serde::{de::DeserializeOwned, Serialize};

use crate::{
    bot::Bot,
    requests::{HasPayload, Payload, Request, ResponseResult},
    RequestError,
};

/// A ready-to-send Telegram request whose payload is sent using [JSON].
///
/// [JSON]: https://core.telegram.org/bots/api#making-requests
#[must_use = "requests do nothing until sent"]
pub struct JsonRequest<P> {
    bot: Bot,
    payload: P,
}

impl<P> JsonRequest<P> {
    pub const fn new(bot: Bot, payload: P) -> Self {
        Self { bot, payload }
    }
}

impl<P> Request for JsonRequest<P>
where
    // FIXME(waffle):
    //   this is required on stable because of
    //   https://github.com/rust-lang/rust/issues/76882
    //   when it's resolved or `type_alias_impl_trait` feature
    //   stabilized, we should remove 'static restriction
    //
    // (though critically, currently we have no
    // non-'static payloads)
    P: 'static,
    P: Payload + Serialize,
    P::Output: DeserializeOwned,
{
    type Err = RequestError;
    type Send = Send<P>;
    type SendRef = SendRef<P>;

    fn send(self) -> Self::Send {
        Send::new(self)
    }

    fn send_ref(&self) -> Self::SendRef {
        SendRef::new(self)
    }
}

impl<P> HasPayload for JsonRequest<P>
where
    P: Payload,
{
    type Payload = P;
}

impl<P> AsMut<P> for JsonRequest<P> {
    fn as_mut(&mut self) -> &mut P {
        &mut self.payload
    }
}

impl<P> AsRef<P> for JsonRequest<P> {
    fn as_ref(&self) -> &P {
        &self.payload
    }
}

impl<P: Payload + Serialize> core::ops::Deref for JsonRequest<P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        self.payload_ref()
    }
}

impl<P: Payload + Serialize> core::ops::DerefMut for JsonRequest<P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.payload_mut()
    }
}

req_future! {
    def: |it: JsonRequest<U>| {
        it.bot.execute_json(&it.payload)
    }
    pub Send<U> (inner0) -> ResponseResult<U::Output>
    where
        U: 'static,
        U: Payload + Serialize,
        U::Output: DeserializeOwned,
}

req_future! {
    def: |it: &JsonRequest<U>| {
        it.bot.execute_json(&it.payload)
    }
    pub SendRef<U> (inner1) -> ResponseResult<U::Output>
    where
        U: 'static,
        U: Payload + Serialize,
        U::Output: DeserializeOwned,
}
