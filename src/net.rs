//! Network-specific API.

pub use self::download::{download_file, download_file_stream, Download};

pub(crate) use self::{
    request::{request_json, request_multipart},
    telegram_response::TelegramResponse,
};

mod download;
mod request;
mod telegram_response;

/// The default Telegram API URL.
pub const TELEGRAM_API_URL: &str = env!("TELEGRAM_API_URL");

/// Constructs a network client from the `TELOXIDE_PROXY` environmental
/// variable.
///
/// This function passes the value of `TELOXIDE_PROXY` into
/// [`reqwest::Proxy::all`], if it exists, otherwise returns the default
/// client.
///
/// ## Note
///
/// The created client will have safe settings, meaning that it will be able to
/// work in long time durations, see the [issue 223].
///
/// [`reqwest::Proxy::all`]: https://docs.rs/reqwest/latest/reqwest/struct.Proxy.html#method.all
/// [issue 223]: https://github.com/teloxide/teloxide/issues/223
///
/// ## Panics
///
/// If `TELOXIDE_PROXY` exists, but isn't correct url.
pub fn client_from_env() -> reqwest::Client {
    use crate::bot::{sound_bot, TELOXIDE_PROXY};
    use reqwest::Proxy;

    let builder = sound_bot();

    match std::env::var(TELOXIDE_PROXY).ok() {
        Some(proxy) => builder.proxy(Proxy::all(&proxy).expect("creating reqwest::Proxy")),
        None => builder,
    }
    .build()
    .expect("creating reqwest::Client")
}

/// Creates URL for making HTTPS requests. See the [Telegram documentation].
///
/// [Telegram documentation]: https://core.telegram.org/bots/api#making-requests
fn method_url(base: reqwest::Url, token: &str, method_name: &str) -> reqwest::Url {
    base.join(&format!(
        "/bot{token}/{method}",
        token = token,
        method = method_name
    ))
    .expect("failed to format url")
}

/// Creates URL for downloading a file. See the [Telegram documentation].
///
/// [Telegram documentation]: https://core.telegram.org/bots/api#file
fn file_url(base: reqwest::Url, token: &str, file_path: &str) -> reqwest::Url {
    base.join(&format!(
        "file/bot{token}/{file}",
        token = token,
        file = file_path
    ))
    .expect("failed to format url")
}

#[cfg(test)]
mod tests {
    use crate::net::*;

    #[test]
    fn method_url_test() {
        let url = method_url(
            reqwest::Url::parse(TELEGRAM_API_URL).unwrap(),
            "535362388:AAF7-g0gYncWnm5IyfZlpPRqRRv6kNAGlao",
            "methodName",
        );

        assert_eq!(
            url.as_str(),
            "https://api.telegram.org/bot535362388:AAF7-g0gYncWnm5IyfZlpPRqRRv6kNAGlao/methodName"
        );
    }

    #[test]
    fn file_url_test() {
        let url = file_url(
            reqwest::Url::parse(TELEGRAM_API_URL).unwrap(),
            "535362388:AAF7-g0gYncWnm5IyfZlpPRqRRv6kNAGlao",
            "AgADAgADyqoxG2g8aEsu_KjjVsGF4-zetw8ABAEAAwIAA20AA_8QAwABFgQ",
        );

        assert_eq!(
            url.as_str(),
            "https://api.telegram.org/file/bot535362388:AAF7-g0gYncWnm5IyfZlpPRqRRv6kNAGlao/AgADAgADyqoxG2g8aEsu_KjjVsGF4-zetw8ABAEAAwIAA20AA_8QAwABFgQ"
        );
    }
}
