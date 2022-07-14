use std::path::Path;

use reqwest::{
    header::{HeaderValue, CONTENT_TYPE},
    Body, Method, Request, StatusCode, Url,
};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, BufReader};

pub mod errors;
use super::{customisations::Prompt, TextToSpeech};
use errors::*;
#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
/// Information about all speaker models for the service instance
pub struct Speaker {
    /// The speaker ID (GUID) of the speaker
    pub speaker_id: String,
    /// The user-defined name of the speaker
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
/// Provides information about the prompts that are defined for a specified speaker in the custom models that are owned by a specified service instance
pub struct SpeakerCustomModel {
    pub customization_id: String,
    pub prompts: Vec<Prompt>,
}

impl TextToSpeech<'_> {
    /// Lists information about all speaker models that are defined for a service instance. The information includes the speaker ID and speaker name of each defined speaker. You must use credentials for the instance of a service to list its speakers. Speaker models and the custom prompts with which they are used are supported only for use with US English custom models and voices.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ibm_watson::{
    /// #     auth::IamAuthenticator,
    /// #     tts::{voices::WatsonVoice, TextToSpeech},
    /// # };
    /// # async fn foo()-> Result<(), Box<dyn std::error::Error>> {
    /// # let auth = IamAuthenticator::new("api_key").await?;
    /// # let tts = TextToSpeech::new(&auth, "service_url");
    /// let speakers = tts.list_speaker_models().await?;
    /// println!("Speakers count: {}", speakers.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_speaker_models(&self) -> Result<Vec<Speaker>, ListSpeakersError> {
        let mut url = Url::parse(self.service_url).unwrap();
        Self::set_speakers_path(&mut url);
        let req = Request::new(Method::GET, url);
        let client = self.get_client();
        let response = client
            .execute(req)
            .await
            .map_err(|e| ListSpeakersError::ConnectionError(e.to_string()))?;
        match response.status() {
            StatusCode::OK => {
                #[derive(Deserialize)]
                struct Root {
                    voices: Vec<Speaker>,
                }
                let root: Root = response.json().await.unwrap();

                Ok(root.voices)
            }
            StatusCode::BAD_REQUEST => Err(ListSpeakersError::BadRequest400),
            StatusCode::INTERNAL_SERVER_ERROR => Err(ListSpeakersError::InternalServerError500),
            StatusCode::SERVICE_UNAVAILABLE => Err(ListSpeakersError::ServiceUnavailable503),
            _ => {
                unreachable!()
            }
        }
    }

    fn set_speakers_path(uri: &mut Url) {
        uri.set_path("v1/speakers");
    }

    /// Creates a new speaker model, which is an optional enrollment token for users who are to add prompts to custom models. A speaker model contains information about a user's voice. The service extracts this information from a WAV audio sample that you pass as the body of the request. Associating a speaker model with a prompt is optional, but the information that is extracted from the speaker model helps the service learn about the speaker's voice
    ///
    /// # Parameters
    /// * `speaker_name` - The name of the speaker that is to be added to the service instance
    ///     * Include a maximum of 49 characters in the name
    ///     * Include only alphanumeric characters and _ (underscores) in the name
    ///     * Do not include XML sensitive characters (double quotes, single quotes, ampersands, angle brackets, and slashes) in the name
    ///     * Do not use the name of an existing speaker that is already defined for the service instance
    /// * `audio_file` - An enrollment audio file that contains a sample of the speaker’s voice
    ///     * The enrollment audio must be in WAV format and must have a minimum sampling rate of 16 kHz. The service accepts audio with higher sampling rates. It transcodes all audio to 16 kHz before processing it
    ///     * The length of the enrollment audio is limited to 1 minute. Speaking one or two paragraphs of text that include five to ten sentences is recommended
    ///
    /// # Returns
    ///
    /// The [`speaker_id`] for the newly created speaker
    ///
    /// # Example
    /// ``` no_run
    /// # use ibm_watson::{
    /// #     auth::IamAuthenticator,
    /// #     tts::{voices::WatsonVoice, TextToSpeech,
    /// #     customisations::Prompt},
    /// # };
    /// # async fn foo()-> Result<(), Box<dyn std::error::Error>> {
    /// # let auth = IamAuthenticator::new("api_key").await?;
    /// # let tts = TextToSpeech::new(&auth, "service_url");
    /// let file_path = std::path::Path::new("/home/user/audio.wav");
    /// let speaker_id = tts.create_speaker_model("speaker_one", &file_path).await?;
    /// println!("created speaker: {}", speaker_id);
    /// # Ok(())
    /// # }
    /// ```
    /// [`speaker_id`]: crate::tts::speaker_models::Speaker::speaker_id
    pub async fn create_speaker_model(
        &self,
        speaker_name: impl AsRef<str>,
        audio_file: impl AsRef<Path>,
    ) -> Result<String, CreateSpeakerError> {
        let wav_file = audio_file.as_ref();
        let file = tokio::fs::OpenOptions::new()
            .read(true)
            .open(&wav_file)
            .await
            .map_err(|e| CreateSpeakerError::FileReadError(e.to_string()))?;

        let mut buf_reader = BufReader::new(file);
        let mut buffer = Vec::new();
        buf_reader
            .read_to_end(&mut buffer)
            .await
            .map_err(|e| CreateSpeakerError::FileReadError(e.to_string()))?;

        let mut url = Url::parse(self.service_url).unwrap();
        Self::set_speakers_path(&mut url);
        url.set_query(Some(&format!("speaker_name={}", speaker_name.as_ref())));
        let body = Body::from(buffer);
        let client = self.get_client();
        let response = client
            .post(url)
            .header(CONTENT_TYPE, HeaderValue::from_static("audio/wav"))
            .body(body)
            .send()
            .await
            .unwrap();
        match response.status() {
            StatusCode::CREATED => {
                #[derive(Deserialize)]
                struct Foo {
                    speaker_id: String,
                }
                let response: Foo = response.json().await.unwrap();
                Ok(response.speaker_id)
            }
            StatusCode::BAD_REQUEST => Err(CreateSpeakerError::BadRequest400),
            StatusCode::UNSUPPORTED_MEDIA_TYPE => Err(CreateSpeakerError::UnsupportedMediaType415),
            StatusCode::INTERNAL_SERVER_ERROR => Err(CreateSpeakerError::InternalServerError500),
            StatusCode::SERVICE_UNAVAILABLE => Err(CreateSpeakerError::ServiceUnavailable503),
            _ => unreachable!(),
        }
    }

    /// Gets information about all prompts that are defined by a specified speaker for all custom models that are owned by a service instance. The information is grouped by the customization IDs of the custom models. For each custom model, the information lists information about each prompt that is defined for that custom model by the speaker. You must use credentials for the instance of the service that owns a speaker model to list its prompts. Speaker models and the custom prompts with which they are used are supported only for use with US English custom models and voices
    ///
    /// # Parameters
    /// * `speaker_id` - The speaker ID (GUID) of the speaker model. You must make the request with service credentials for the instance of the service that owns the speaker model
    ///
    /// # Example
    /// ``` no_run
    /// # use ibm_watson::{
    /// #     auth::IamAuthenticator,
    /// #     tts::{voices::WatsonVoice, TextToSpeech,
    /// #     customisations::Prompt},
    /// # };
    /// # async fn foo()-> Result<(), Box<dyn std::error::Error>> {
    /// # let auth = IamAuthenticator::new("api_key").await?;
    /// # let tts = TextToSpeech::new(&auth, "service_url");
    /// let speaker = tts.get_speaker_model("speaker_id").await?;
    /// println!("Speaker: {:#?}", speaker);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_speaker_model(
        &self,
        speaker_id: impl AsRef<str>,
    ) -> Result<SpeakerCustomModel, GetSpeakerError> {
        let mut url = Url::parse(self.service_url).unwrap();
        url.set_path(&format!("v1/speakers/{}", speaker_id.as_ref()));
        let req = Request::new(Method::GET, url);
        let client = self.get_client();
        let response = client
            .execute(req)
            .await
            .map_err(|e| GetSpeakerError::ConnectionError(e.to_string()))?;
        assert_eq!(response.status(), 200);
        match response.status() {
            StatusCode::OK => {
                let root: SpeakerCustomModel = response.json().await.unwrap();

                Ok(root)
            }
            StatusCode::BAD_REQUEST => Err(GetSpeakerError::BadRequest400),
            StatusCode::UNAUTHORIZED => Err(GetSpeakerError::Unauthorised401(
                speaker_id.as_ref().to_owned(),
            )),
            StatusCode::NOT_MODIFIED => Err(GetSpeakerError::NotModified304),
            StatusCode::INTERNAL_SERVER_ERROR => Err(GetSpeakerError::InternalServerError500),
            StatusCode::SERVICE_UNAVAILABLE => Err(GetSpeakerError::ServiceUnavailable503),
            _ => {
                unreachable!()
            }
        }
    }

    /// Deletes an existing speaker model from the service instance. The service deletes the enrolled speaker with the specified speaker ID. You must use credentials for the instance of the service that owns a speaker model to delete the speaker
    /// # Parameters
    ///
    /// `speaker_id` - The speaker ID (GUID) of the speaker model. You must make the request with service credentials for the instance of the service that owns the speaker model
    ///
    /// # Example
    /// ``` no_run
    /// # use ibm_watson::{
    /// #     auth::IamAuthenticator,
    /// #     tts::{voices::WatsonVoice, TextToSpeech},
    /// # };
    /// # async fn foo()-> Result<(), Box<dyn std::error::Error>> {
    /// # let auth = IamAuthenticator::new("api_key").await?;
    /// # let tts = TextToSpeech::new(&auth, "service_url");
    /// if tts.delete_speaker_model("speaker-id").await.is_ok() {
    ///     println!("speaker deleted");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    /// [`model`]: crate::tts::customisations::Model
    pub async fn delete_speaker_model(
        &self,
        speaker_id: impl AsRef<str>,
    ) -> Result<(), DeleteSpeakerError> {
        let mut url = Url::parse(self.service_url).unwrap();
        url.set_path(&format!("v1/speakers/{}", speaker_id.as_ref()));
        let req = Request::new(Method::DELETE, url);
        let client = self.get_client();
        let response = client
            .execute(req)
            .await
            .map_err(|e| DeleteSpeakerError::ConnectionError(e.to_string()))?;
        match response.status() {
            StatusCode::NO_CONTENT => Ok(()),
            StatusCode::BAD_REQUEST => Err(DeleteSpeakerError::BadRequest400(
                speaker_id.as_ref().to_owned(),
            )),
            StatusCode::INTERNAL_SERVER_ERROR => Err(DeleteSpeakerError::InternalServerError500),
            StatusCode::SERVICE_UNAVAILABLE => Err(DeleteSpeakerError::ServiceUnavailable503),
            StatusCode::UNAUTHORIZED => Err(DeleteSpeakerError::Unauthorised401(
                speaker_id.as_ref().to_owned(),
            )),
            _ => {
                unreachable!()
            }
        }
    }
}

