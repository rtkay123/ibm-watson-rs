#![feature(doc_cfg)]
//! A wrapper for interacting with IBM Watson's API
//!
//! # Usage
//!
//! Add `ibm-watson` to your `Cargo.toml`
//!
//! ```toml
//! [dependencies]
//! ibm-watson = { git = "https://github.com/kawaki-san/ibm-watson-rs }
//! ```
//!
//! By default, the crate has no enabled features, so you have to select which ever
//! ones you would like to use. For example, to use the `Text to Speech` service,
//! you have to enable the feature `tts`:
//!
//! ```toml
//! [dependencies]
//! ibm-watson = { git = "https://github.com/kawaki-san/ibm-watson-rs, features = [ "tts" ] }
//! ```
//!
//! # Example and API Usage
//!
//! For interacting with the Text To Speech API:
//!
//! ```sh
//! cargo run --example tts --features="tts" -- --api-key "my_api_key" --service-url "my_service_url"
//! ```
//!
//! This runs an example that prints all available (Text To Speech) voices and
//! synthesises some sample text.
//!
//! # License
//!
//! This crate is licensed under either of:
//!
//! - Apache License, Version 2.0 [`LICENSE-APACHE`](LICENSE-APACHE) or
//!   `<http://www.apache.org/licenses/LICENSE-2.0)>`
//! - MIT license [`LICENSE-MIT`](LICENSE-MIT) or
//!   `<http://opensource.org/licenses/MIT)>` at your option.
//!
//! # Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted
//! for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
//! dual licensed as above, without any additional terms or conditions.
#[warn(
    missing_debug_implementations,
    missing_docs,
    rustdoc::broken_intra_doc_links,
   // unreachable_pub,
   // rustdoc::missing_doc_code_examples
)]

/// Retrieve an IAM access token to use for authentication with your IBM Watson services
///
/// # Example
///
/// ``` no_run
/// # use ibm_watson::{
/// #     auth::IamAuthenticator,
/// #     tts::{voices::WatsonVoice, TextToSpeech},
/// # };
/// # async fn foo()-> Result<(), Box<dyn std::error::Error>> {
/// let auth = IamAuthenticator::new("api_key").await?;
/// # Ok(())
/// # }
/// ```
pub mod auth;
/// Interact with the IBM Watson™ Text to Speech service
#[cfg(feature = "tts")]
#[doc(cfg(feature = "tts"))]
#[path = "text-to-speech/mod.rs"]
pub mod tts;

#[cfg(test)]
mod tests;
