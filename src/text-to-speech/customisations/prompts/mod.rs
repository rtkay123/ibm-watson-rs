use std::path::Path;

use reqwest::{
    header::{HeaderValue, CONTENT_TYPE},
    multipart::{Form, Part},
    Method, Request, StatusCode, Url,
};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, BufReader};

use crate::tts::TextToSpeech;

use super::errors::{AddPromptError, ListPromptsError};

#[derive(Clone, Debug, Eq, PartialEq, Default, Serialize, Deserialize)]
struct OuterPrompt {
    #[serde(rename = "prompt")]
    pub prompt: String,
    #[serde(rename = "prompt_id")]
    pub prompt_id: String,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "speaker_id", skip_serializing_if = "Option::is_none")]
    pub speaker_id: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Serialize, Deserialize)]
/// Defines information about the prompt in a custom model
pub struct Prompt {
    /// The user-specified text of the prompt
    pub prompt: String,
    #[serde(rename = "prompt_id")]
    /// The user-specified identifier (name) of the prompt
    pub prompt_id: String,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<PromptStatus>,
    /// If the status of the prompt is failed, an error message that describes the reason for the failure. The field is omitted if no error occurred
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// The speaker ID (GUID) of the speaker for which the prompt was defined. The field is omitted if no speaker ID was specified
    #[serde(rename = "speaker_id", skip_serializing_if = "Option::is_none")]
    pub speaker_id: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
/// The status of the prompt:
pub enum PromptStatus {
    /// The service received the request to add the prompt and is analyzing the validity of the prompt.
    Processing,
    /// The service successfully validated the prompt, which is now ready for use in a speech synthesis request.
    Available,
    /// The service's validation of the prompt failed. The status of the prompt includes an error field that describes the reason for the failure.
    Failed,
}

impl From<OuterPrompt> for Prompt {
    fn from(prompt: OuterPrompt) -> Self {
        let status = match prompt.status {
            Some(ref val) => match val.as_str() {
                "processing" => Some(PromptStatus::Processing),
                "available" => Some(PromptStatus::Available),
                "failed" => Some(PromptStatus::Failed),
                _ => unreachable!(),
            },
            None => None,
        };
        Self {
            prompt: prompt.prompt,
            prompt_id: prompt.prompt_id,
            status,
            error: prompt.error,
            speaker_id: prompt.speaker_id,
        }
    }
}

impl TextToSpeech<'_> {
    /// Lists information about all custom prompts that are defined for a custom [`model`]. The information includes the [`prompt ID`], [`prompt text`], [`status`], and
    /// optional [`speaker ID`] for each prompt of the custom model. You must use credentials for the instance of the service that owns the custom model.
    /// The same information about all of the prompts for a custom model is also provided by [`get_custom_model()`]. That method provides complete details about a specified
    /// custom model, including its [`language`], [`owner`], [`custom words`], and more. Custom prompts are supported only for use with US English custom models and voices.
    ///
    /// # Parameters
    ///
    /// * `customisation_id` - The customization ID (GUID) of the custom model. You must make the request with credentials for the instance of the service that owns the custom model
    ///
    /// # Example
    /// ``` no_run
    /// # use ibm_watson::{
    /// #     auth::IamAuthenticator,
    /// #     tts::{voices::WatsonVoice, TextToSpeech,
    /// #     customisations::Word},
    /// # };
    /// # async fn foo()-> Result<(), Box<dyn std::error::Error>> {
    /// # let auth = IamAuthenticator::new("api_key").await?;
    /// # let tts = TextToSpeech::new(&auth, "service_url");
    /// let prompts = tts.list_custom_prompts("word").await?;
    /// println!("{:#?}", prompts);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`model`]: crate::tts::customisations::Model
    /// [`prompt ID`]: crate::tts::customisations::Prompt::prompt_id
    /// [`prompt text`]: crate::tts::customisations::Prompt::prompt
    /// [`status`]: crate::tts::customisations::Prompt::status
    /// [`speaker ID`]: crate::tts::customisations::Prompt::speaker_id
    /// [`get_custom_model()`]: crate::tts::TextToSpeech::get_custom_model()
    /// [`language`]: crate::tts::customisations::Model::language
    /// [`owner`]: crate::tts::customisations::Model::owner
    /// [`custom words`]: crate::tts::customisations::Model::words
    ///
    pub async fn list_custom_prompts(
        &self,
        customisation_id: impl AsRef<str>,
    ) -> Result<Vec<Prompt>, ListPromptsError> {
        let mut url = Url::parse(self.service_url).unwrap();
        url.set_path(&format!(
            "v1/customizations/{}/prompts",
            customisation_id.as_ref()
        ));
        let req = Request::new(Method::GET, url);
        let client = self.get_client();
        let response = client
            .execute(req)
            .await
            .map_err(|e| ListPromptsError::ConnectionError(e.to_string()))?;
        match response.status() {
            StatusCode::OK => {
                #[derive(Deserialize, Serialize)]
                struct Root {
                    prompts: Vec<Prompt>,
                }
                let root: Root = response.json().await.unwrap();
                Ok(root.prompts)
            }
            StatusCode::BAD_REQUEST => Err(ListPromptsError::BadRequest400),
            StatusCode::INTERNAL_SERVER_ERROR => Err(ListPromptsError::InternalServerError500),
            StatusCode::SERVICE_UNAVAILABLE => Err(ListPromptsError::ServiceUnavailable503),
            _ => {
                unreachable!()
            }
        }
    }

    /// Adds a custom prompt to a custom model. A prompt is defined by the text that is to be spoken, the audio for that text, a unique user-specified ID for the prompt, and an optional speaker ID. The information is used to generate prosodic data that is not visible to the user. This data is used by the service to produce the synthesized audio upon request. You must use credentials for the instance of the service that owns a custom model to add a prompt to it. You can add a maximum of 1000 custom prompts to a single custom model
    ///
    /// # Parameters
    ///
    /// * `customisation_id` - The customization ID (GUID) of the custom model. You must make the request with credentials for the instance of the service that owns the custom model
    /// * `prompt` - The prompt that is to be added to the custom model
    /// * `audio_file` - An audio file that speaks the text of the prompt with intonation and prosody that matches how you would like the prompt to be spoken
    ///     * The prompt audio must be in WAV format and must have a minimum sampling rate of 16 kHz. The service accepts audio with higher sampling rates. The service transcodes all audio to 16 kHz before processing it
    ///     * The length of the prompt audio is limited to 30 seconds
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
    /// let prompt = Prompt {
    ///     prompt: String::from("foo"),
    ///     prompt_id: String::from("bar"),
    ///     ..Default::default()
    /// };
    /// let _ = tts.add_custom_prompt("cust-id", &prompt, &file_path).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`model`]: crate::tts::customisations::Model
    /// [`prompt ID`]: crate::tts::customisations::Prompt::prompt_id
    /// [`prompt text`]: crate::tts::customisations::Prompt::prompt
    /// [`status`]: crate::tts::customisations::Prompt::status
    /// [`speaker ID`]: crate::tts::customisations::Prompt::speaker_id
    /// [`get_custom_model()`]: crate::tts::TextToSpeech::get_custom_model()
    /// [`language`]: crate::tts::customisations::Model::language
    /// [`owner`]: crate::tts::customisations::Model::owner
    /// [`custom words`]: crate::tts::customisations::Model::words
    ///
    pub async fn add_custom_prompt(
        &self,
        customisation_id: impl AsRef<str>,
        prompt: &Prompt,
        audio_file: impl AsRef<Path>,
    ) -> Result<Prompt, AddPromptError> {
        let audio_file = audio_file.as_ref().to_owned();
        let name = audio_file.clone();
        let f_name = name.file_name();
        let file_name = f_name
            .ok_or_else(|| AddPromptError::FileReadError("Could not read file".to_owned()))?;

        let file_name = file_name.to_string_lossy();
        let file_name = file_name.to_string();
        let file = tokio::fs::OpenOptions::new()
            .read(true)
            .open(&audio_file)
            .await
            .map_err(|e| AddPromptError::FileReadError(e.to_string()))?;
        let mut buf_reader = BufReader::new(file);
        let mut buffer = Vec::new();
        buf_reader
            .read_to_end(&mut buffer)
            .await
            .map_err(|e| AddPromptError::FileReadError(e.to_string()))?;

        let mut url = Url::parse(self.service_url).unwrap();
        url.set_path(&format!(
            "v1/customizations/{}/prompts/{}",
            customisation_id.as_ref(),
            prompt.prompt_id
        ));
        let forms;
        let form = Form::new()
            .text("prompt_text", prompt.prompt.to_owned())
            .part("file", Part::bytes(buffer).file_name(file_name));

        if let Some(speaker) = &prompt.speaker_id {
            forms = form.text("speaker_id", speaker.to_owned());
        } else {
            forms = form;
        };
        let client = self.get_client();
        let response = client
            .post(url)
            .header(
                CONTENT_TYPE,
                HeaderValue::from_static("multipart/form-data"),
            )
            .multipart(forms)
            .send()
            .await
            .unwrap();
        match response.status() {
            StatusCode::CREATED => Ok(response.json().await.unwrap()),
            StatusCode::BAD_REQUEST => Err(AddPromptError::BadRequest400),
            StatusCode::UNAUTHORIZED => Err(AddPromptError::Unauthorised401(
                customisation_id.as_ref().to_string(),
            )),
            StatusCode::UNSUPPORTED_MEDIA_TYPE => Err(AddPromptError::UnsupportedMediaType415),
            StatusCode::INTERNAL_SERVER_ERROR => Err(AddPromptError::InternalServerError500),
            StatusCode::SERVICE_UNAVAILABLE => Err(AddPromptError::ServiceUnavailable503),
            _ => unreachable!(),
        }
    }
}
