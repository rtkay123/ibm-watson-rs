<h1 align="center">IBM Watson</h1>
<p align="center">A wrapper for interacting with IBM Watson's API</p>
<div align="center">
<img alt="Crates.io" style="align: center" src="https://img.shields.io/crates/v/ibm-watson">
<img alt="docs.rs" src="https://img.shields.io/docsrs/ibm-watson">
<img alt="GitHub Workflow Status" src="https://img.shields.io/github/workflow/status/kawaki-san/ibm-watson-rs/Cargo%20Build%20&%20Test%20(with%20Docs)">
<img alt="Crates.io" src="https://img.shields.io/crates/l/ibm-watson">
</div>

## Usage

Add `ibm-watson` to your `Cargo.toml`

```toml
[dependencies]
ibm-watson = "0.1.1"
```

## Feature Flags

This crate uses a set of featue flags to reduce the amount of compiled code. By
default, none of the features are enabled and it is therefore recommended that
you do so for only those services you intend to use.

- `full` - Enables all the features listed below
- `http2` - Enables support of `HTTP/2.0` requests
- `tts` - Enables interacting with the Text To Speech API

## Example

To use the Text To Speech API to synthesise some text with the default options,
enable the `tts` feature

```toml
[dependencies]
ibm-watson = { version = "0.1.1", features = [ "tts" ] }
```

```rust
// Get your IAM access token with the API Key of the particular service you want to use
let auth = IamAuthenticator::new("my_api_key").await?;
// Create a new Text To Speech instance that you will use to interact with the API
let tts = TextToSpeech::new(&auth, "tts-endpoint");
// Call whatever method you would like to use from it
let synth = tts.synthesise("Hello world", None, None).await?;
let mut file = File::create("file.ogg")?;
file.write_all(&synth)?;
```

To perform synthesis with a custom voice and in a different audio format:

```rust
// This sets Kate (United Kingdom) to be the default voice for your requests
tts.set_voice(WatsonVoice::EnGbKateV3);
// set the format to MP3 with a sample rate of 44100khz
let format = AudioFormat::AudioMp3 {
// If `None` is passed, then the crate will default to 22050.
    sample_rate: Some(44100),
};
let synth = tts.synthesise("Hello world", Some(format), None).await?;
```

There are examples ready to get you started quickly. To run an example that uses
the Text To Speech service to print available voices and then synthesise the
text you entered to a file:

```sh
cargo run --example tts --features="tts" -- -a "my_api_key" -s "my_service_url" -t "Greetings from Rust"
```

## License

This crate is licensed under either of:

- Apache License, Version 2.0
- MIT License

at your option.

[Apache License, Version 2.0]: http://www.apache.org/licenses/LICENSE-2.0
[MIT License]: http://opensource.org/licenses/MIT

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

_This is currently unofficial, experimental software that is under development.
As such, contributions are welcome._ _This crate's documentation is sourced from
IBM Watson's official
[API Documentation](https://developer.ibm.com/components/watson-apis/apis). If
you would like to know more about Watson's API, that would be a good place to
start._
