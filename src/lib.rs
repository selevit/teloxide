#![doc(
    html_logo_url = "https://github.com/teloxide/teloxide/raw/dev/logo.svg",
    html_favicon_url = "https://github.com/teloxide/teloxide/raw/dev/ICON.png"
)]
#![allow(clippy::match_bool)]

pub use bot::Bot;
pub use errors::{DownloadError, RequestError};

mod errors;
mod network;

mod bot;
pub mod dispatching;
pub mod requests;
pub mod types;
