use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Default, Serialize, Deserialize)]
pub struct Prompt {
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
