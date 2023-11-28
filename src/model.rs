/// This module contains the data model for the lexicon.
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LexicographicResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    pub lang_code: LangCode,
    #[serde(alias = "entry")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub entries: Vec<Entry>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(transparent)]
pub struct LangCode(pub String);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Entry { 
    pub headword: HeadwordString,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub homograph_number: Option<u32>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub part_of_speech: Vec<PartOfSpeech>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub label: Vec<Label>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub pronunciation: Vec<Pronunciation>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub inflected_from: Vec<InflectedForm>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "senses")]
    pub sense: Vec<Sense>
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct HeadwordString(pub Vec<HeadwordStringPart>);

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum HeadwordStringPart {
    Text(String),
    PlaceholderMarker(String)
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PartOfSpeech {
    pub tag: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
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

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct TextString(pub Vec<TextStringPart>);

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum TextStringPart {
    Text(String),
    HeadwordMarker(String),
    CollocateMarker(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct Definition {
    #[serde(rename = "$value")]
    pub text: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub definition_type: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    pub tag: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Pronunciation {
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
#[serde(rename_all = "camelCase")]
pub struct Transcription {
    #[serde(rename = "$value")]
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub scheme: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Example {
    pub text: TextString,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub source_identity: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub source_elaboration: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub label: Vec<Label>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sound_file: Option<String>,
}
