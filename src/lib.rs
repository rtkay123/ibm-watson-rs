#![feature(doc_cfg)]
//! An wrapper for interacting with IBM Watson's API
//!
//! Please have a look at the IBM Watson [`API`] if you would like to know how things work behind
//! the scenes:
//!
//! [`API`]: https://developer.ibm.com/components/watson-apis/apis
//!
//! # Usage
//!
//! Add `ibm-watson` to your `Cargo.toml`
//!
//! ```toml
//! [dependencies]
//! ibm-watson = "0.1.1"
//! ```
//!
//! # Feature Flags
//!
//! This crate uses a set of featue flags to reduce the amount of compiled code. By default, none
//! of the features are enabled and it is therefore recommended that you do so for only those
//! services you intend to use.
//!
//! * `full` - Enables all the features listed below
//! * `http2` - Enables support of `HTTP/2.0` requests
//! * `tts` - Enables interacting with the Text To Speech API
//!
//! # Example
//!
//! To use the Text To Speech API to synthesise some text with the default options, enable the `tts` feature
//!
//! ```toml
//! [dependencies]
//! ibm-watson = { version = "0.1.1", features = [ "tts" ] }
//! ```
//!
//! ``` no_run
//!# use std::{fs::File, io::Write};
//!# use ibm_watson::{
//!#     auth::IamAuthenticator,
//!#     tts::TextToSpeech
//!# };
//! # #[tokio::main]
//! # async fn main() {
//! // Get your IAM access token with the API Key of the particular service you want to use
//! let auth = IamAuthenticator::new("my_api_key").await.unwrap();
//! // Create a new Text To Speech instance that you will use to interact with the API
//! let tts = TextToSpeech::new(&auth, "tts-endpoint");
//! // Call whatever method you would like to use from it
//! let synth = tts.synthesise("Hello world", None, None).await.unwrap();
//! let mut file = File::create("file.ogg").unwrap();
//! file.write_all(&synth).unwrap();
//! # }
//! ```
//!
//! To perform synthesis with a custom voice and in a different audio format:
//!
//! ``` no_run
//!# use std::{fs::File, io::Write};
//!# use ibm_watson::{
//!#     auth::IamAuthenticator,
//!#     tts::{synthesis::AudioFormat, TextToSpeech,
//!#     voices::WatsonVoice},
//!# };
//! # #[tokio::main]
//! # async fn main() {
//! # let auth = IamAuthenticator::new("my_api_key").await.unwrap();
//! # let mut tts = TextToSpeech::new(&auth, "tts-endpoint");
//! // This sets Kate (United Kingdom) to be the default voice for your requests
//! tts.set_voice(WatsonVoice::EnGbKateV3);
//! // set the format to MP3 with a sample rate of 44100khz
//! let format = AudioFormat::AudioMp3 {
//! // If `None` is passed, then the crate will default to 22050.
//!     sample_rate: Some(44100),
//! };
//! let synth = tts.synthesise("Hello world", Some(format), None).await.unwrap();
//! # }
//! ```
//!
//! To run a quick demonstration that prints all voices available for usage and synthesises "Hello
//! world":
//!
//! ```sh
//! cargo run --example tts --features="tts" -- --api-key "my_api_key" --service-url "my_service_url"
//! ```
//!
//! # License
//!
//! This crate is licensed under either of:
//!
//! - Apache License, Version 2.0
//! - MIT License
//!
//! at your option.
//!
//! [Apache License, Version 2.0]: http://www.apache.org/licenses/LICENSE-2.0
//!
//! [MIT License]: http://opensource.org/licenses/MIT
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
