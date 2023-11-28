/// This module contains the data model for the lexicon.
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct LexicographicResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    pub lang_code: LangCode,
    #[serde(alias = "entry")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub entries: Vec<Entry>,
    #[serde(alias = "translationLanguage")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub translation_languages: Vec<TranslationLanguage>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(transparent)]
pub struct LangCode(pub String);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Entry { 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub headword: HeadwordString,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub homograph_number: Option<u32>,
    #[serde(alias = "partOfSpeech")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub parts_of_speech: Vec<PartOfSpeech>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "label")]
    pub labels: Vec<Label>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "pronunciation")]
    pub pronunciations: Vec<Pronunciation>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "inflectedForm")]
    pub inflected_forms: Vec<InflectedForm>,
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
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct PartOfSpeech {
    pub tag: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct InflectedForm {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "label")]
    pub labels: Vec<Label>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "pronunciation")]
    pub pronunciations: Vec<Pronunciation>,
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
#[serde(deny_unknown_fields)]
pub struct Sense {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub indicator: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "label")]
    pub labels: Vec<Label>,
    #[serde(alias = "definition")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub definitions: Vec<Definition>,
    #[serde(alias = "example")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub examples: Vec<Example>,
    #[serde(alias = "headwordExplanation")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub headword_explanations: Vec<HeadwordExplanation>,
    #[serde(alias = "headwordTranslation")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub headword_translations: Vec<HeadwordTranslation>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Definition {
    #[serde(rename = "$value")]
    pub text: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub definition_type: Vec<String>
}

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Label {
    pub tag: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Pronunciation {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sound_file: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "transcription")]
    pub transcriptions: Vec<Transcription>,
    #[serde(alias = "label")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub labels: Vec<Label>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Transcription {
    #[serde(alias = "$value")]
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub scheme: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Example {
    pub text: TextString,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub source_identity: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub source_elaboration: Vec<String>,
    #[serde(alias = "label")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub labels: Vec<Label>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sound_file: Option<String>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TranslationLanguage {
    pub lang_code: LangCode,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct HeadwordTranslation {
    pub text: TextString,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub lang_code: Option<LangCode>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "partOfSpeech")]
    pub parts_of_speech: Vec<PartOfSpeech>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "label")]
    pub labels: Vec<Label>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "pronunciation")]
    pub pronunciations: Vec<Pronunciation>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "inflectedForm")]
    pub inflected_forms: Vec<InflectedForm>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct HeadwordExplanation {
    pub text: TextString,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub lang_code: Option<LangCode>,
 }

