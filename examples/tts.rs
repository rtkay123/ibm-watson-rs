use std::{fs::File, io::Write};

use ibm_watson::{
    auth::IamAuthenticator,
    tts::{voices::WatsonVoice, TextToSpeech},
};

use clap::Parser;

/// Interacting with the IBM Watson Text To Speech API
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Your API Key for that particular service
    #[clap(short, long, value_parser)]
    api_key: String,
    /// The Watson service url
    #[clap(short, long, value_parser)]
    service_url: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let auth = IamAuthenticator::new(args.api_key).await.unwrap();
    let tts = TextToSpeech::new(&auth, &args.service_url);
    let voices = tts.list_voices().await.unwrap();
    println!("{:#?}", voices);
    let voice = tts.get_voice(WatsonVoice::EnGbKateV3, None).await.unwrap();
    println!("{:#?}", voice);
    let synth = tts.synthesise("Hello world", None, None).await.unwrap();
    let mut file = File::create("file.ogg").unwrap();
    file.write_all(&synth).unwrap();
}
