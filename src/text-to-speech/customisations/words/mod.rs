use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Word {
    /// the word for the custom model. the maximum length of a word is 49 characters.
    #[serde(rename = "word")]
    pub word: String,
    /// the phonetic or sounds-like translation for the word. a phonetic translation is based on the ssml format for representing the phonetic string of a word either as an ipa or ibm spr translation. the arabic, chinese, dutch, australian english, and korean languages support only ipa. a sounds-like translation consists of one or more words that, when combined, sound like the word. the maximum length of a translation is 499 characters.
    #[serde(rename = "translation")]
    pub translation: String,
    /// japanese only. the part of speech for the word. the service uses the value to produce the correct intonation for the word. you can create only a single entry, with or without a single part of speech, for any word; you cannot create multiple entries with different parts of speech for the same word.
    #[serde(rename = "part_of_speech", skip_serializing_if = "Option::is_none")]
    pub part_of_speech: Option<String>,
}
