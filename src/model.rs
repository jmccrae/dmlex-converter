/// This module contains the data model for the lexicon.
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LexicographicResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(rename = "langCode")]
    pub lang_code: LangCode,
    #[serde(alias = "entries")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub entry: Vec<Entry>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(transparent)]
pub struct LangCode(pub String);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Entry { 
    pub headword: String,
    #[serde(rename = "homographNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub homograph_number: Option<u32>,
    #[serde(rename = "partOfSpeech")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub part_of_speech: Vec<PartOfSpeech>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub label: Vec<Label>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub pronunciation: Vec<Pronunciation>,
    #[serde(rename = "inflectedForm")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub inflected_from: Vec<InflectedForm>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "senses")]
    pub sense: Vec<Sense>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PartOfSpeech {
    pub tag: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InflectedForm {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub label: Vec<Label>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub pronunciation: Vec<Pronunciation>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Sense {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub indicator: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub label: Vec<Label>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub definition: Vec<Definition>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub example: Vec<Example>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Definition {
    #[serde(rename = "$value")]
    pub text: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "definition_type")]
    #[serde(default)]
    pub definition_type: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Label {
    pub tag: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Pronunciation {
    #[serde(rename = "soundFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sound_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub transcription: Option<Transcription>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub label: Vec<Label>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Transcription {
    #[serde(rename = "$value")]
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub scheme: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Example {
    pub text: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "sourceIdentity")]
    #[serde(default)]
    pub source_identity: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "sourceElaboration")]
    #[serde(default)]
    pub source_elaboration: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub label: Vec<Label>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "soundFile")]
    #[serde(default)]
    pub sound_file: Option<String>,
}
