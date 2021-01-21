/// Payload of a request.
///
/// Simply speaking, structures implementing this trait represent arguments of
/// a Telegram bot API method.
///
/// Also, this trait provides some additional information needed to send a
/// request to Telegram.
#[cfg_attr(all(docsrs, feature = "nightly"), doc(spotlight))]
pub trait Payload: AsRef<Self> + AsMut<Self> {
    /// The return type of a Telegram method.
    ///
    /// Note: it should not include `Result` wrappers (e.g. it should be simply
    /// [`Message`], [`True`] or something else).
    ///
    /// [`Message`]: crate::types::Message
    /// [`True`]: crate::types::True
    type Output;

    /// Name of a Telegram method.
    ///
    /// It is case insensitive, though must not include underscores. (e.g.
    /// `GetMe`, `GETME`, `getme`, `getMe` are ok, but `get_me` is not ok).
    const NAME: &'static str;
}
