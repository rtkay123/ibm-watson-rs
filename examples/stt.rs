use ibm_watson::{auth::IamAuthenticator, stt::SpeechToText};

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
    let stt = SpeechToText::new(&auth, &args.service_url);
    let models = stt.list_models().await.unwrap();
    println!("{:#?}", models);
}
