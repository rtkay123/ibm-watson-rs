use serde::{Deserialize, Serialize};

use super::{custom_prompts::Prompt, custom_words::Word};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CustomModel {
    /// the customization id (guid) of the custom model. the create a custom model method returns only this field. it does not not return the other fields of this object.
    #[serde(rename = "customization_id")]
    pub customization_id: String,
    /// the name of the custom model.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// the language identifier of the custom model (for example, en-us).
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// the guid of the credentials for the instance of the service that owns the custom model.
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// the date and time in coordinated universal time (utc) at which the custom model was created. the value is provided in full iso 8601 format (yyyy-mm-ddthh:mm:ss.stzd)
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// the date and time in coordinated universal time (utc) at which the custom model was last modified. the created and updated fields are equal when a model is first added but has yet to be updated. the value is provided in full iso 8601 format (yyyy-mm-ddthh:mm:ss.stzd).
    #[serde(rename = "last_modified", skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    /// the description of the custom model.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// an array of word objects that lists the words and their translations from the custom model. the words are listed in alphabetical order, with uppercase letters listed before lowercase letters. the array is empty if no words are defined for the custom model. this field is returned only by the get a custom model method.
    #[serde(rename = "words", skip_serializing_if = "Option::is_none")]
    pub words: Option<Vec<Word>>,
    /// an array of prompt objects that provides information about the prompts that are defined for the specified custom model. the array is empty if no prompts are defined for the custom model. this field is returned only by the get a custom model method.
    #[serde(rename = "prompts", skip_serializing_if = "Option::is_none")]
    pub prompts: Option<Vec<Prompt>>,
}
