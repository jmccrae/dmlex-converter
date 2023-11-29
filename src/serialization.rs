/// Any serialization specific code should be placed in this module.
use serde::Deserialize;
use serde::de::{Visitor, Deserializer};
use std::fmt;
use crate::model::*;

struct HeadwordStringVisitor;

impl<'de> Visitor<'de> for HeadwordStringVisitor {
    type Value = HeadwordString;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string or a placeholderMarker")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(HeadwordString(vec![HeadwordStringPart::Text(value.to_owned())]))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut content = Vec::new();
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "placeholderMarker" => {
                    content.push(HeadwordStringPart::PlaceholderMarker(map.next_value()?));
                }
                "text" => {
                    content.push(HeadwordStringPart::Text(map.next_value()?));
                }
                "$value" => {
                    content.push(HeadwordStringPart::Text(map.next_value()?));
                }
                _ => {
                    return Err(serde::de::Error::unknown_field(
                        key.as_str(),
                        &["placeholderMarker", "text", "$value"],
                    ))
                }
            }
        }
        Ok(HeadwordString(content))
    }
}

impl<'de> Deserialize<'de> for HeadwordString {
    fn deserialize<D>(deserializer: D) -> Result<HeadwordString, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut headword_string = deserializer.deserialize_any(HeadwordStringVisitor)?;
        if headword_string.0.is_empty() {
            Err(serde::de::Error::custom("HeadwordString must not be empty"))
        } else {
            match &headword_string.0[0] {
                HeadwordStringPart::PlaceholderMarker(m) => {
                   headword_string.0[0] = HeadwordStringPart::PlaceholderMarker(m.trim_start().to_owned()); 
                },
                HeadwordStringPart::Text(ref t) => {
                    headword_string.0[0] = HeadwordStringPart::Text(t.trim_start().to_owned());
                }
            }
            let last_idx = headword_string.0.len() - 1;
            match &headword_string.0[last_idx] {
                HeadwordStringPart::PlaceholderMarker(m) => {
                   headword_string.0[last_idx] = HeadwordStringPart::PlaceholderMarker(m.trim_end().to_owned()); 
                },
                HeadwordStringPart::Text(ref t) => {
                    headword_string.0[last_idx] = HeadwordStringPart::Text(t.trim_end().to_owned());
                }
            }
            Ok(headword_string)
        }
    }
}

struct TextStringVisitor;

impl<'de> Visitor<'de> for TextStringVisitor {
    type Value = TextString;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string or a placeholderMarker")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(TextString(vec![TextStringPart::Text(value.to_owned())]))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut content = Vec::new();
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "headwordMarker" => {
                    content.push(TextStringPart::HeadwordMarker(map.next_value()?));
                }
                "collocateMarker" => {
                    let collocates : std::collections::HashMap<String, String> = map.next_value()?;
                    content.push(TextStringPart::CollocateMarker(
                            collocates.get("$value").map(|s| s.clone()).unwrap_or_else(|| "".to_owned()),
                            collocates.get("lemma").map(|s| s.clone()),
                            Vec::new()));
                }
                "text" => {
                    content.push(TextStringPart::Text(map.next_value()?));
                }
                "$value" => {
                    content.push(TextStringPart::Text(map.next_value()?));
                }
                _ => {
                    return Err(serde::de::Error::unknown_field(
                        key.as_str(),
                        &["headwordMarker", "collocateMarker", "text", "$value"],
                    ))
                }
            }
        }
        Ok(TextString(content))
    }
}

impl<'de> Deserialize<'de> for TextString {
    fn deserialize<D>(deserializer: D) -> Result<TextString, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut string = deserializer.deserialize_any(TextStringVisitor)?;
        if string.0.is_empty() {
            Err(serde::de::Error::custom("TextString must not be empty"))
        } else {
            match &string.0[0] {
                TextStringPart::HeadwordMarker(m) => {
                   string.0[0] = TextStringPart::HeadwordMarker(m.trim_start().to_owned()); 
                },
                TextStringPart::CollocateMarker(m, lemma, labels) => {
                   string.0[0] = TextStringPart::CollocateMarker(m.trim_start().to_owned(), lemma.clone(), labels.clone()); 
                },
                TextStringPart::Text(ref t) => {
                    string.0[0] = TextStringPart::Text(t.trim_start().to_owned());
                }
            }
            let last_idx = string.0.len() - 1;
            match &string.0[last_idx] {
                TextStringPart::HeadwordMarker(m) => {
                   string.0[last_idx] = TextStringPart::HeadwordMarker(m.trim_end().to_owned()); 
                },
                TextStringPart::CollocateMarker(m, lemma, labels) => {
                   string.0[last_idx] = TextStringPart::CollocateMarker(m.trim_end().to_owned(), lemma.clone(), labels.clone()); 
                },
                TextStringPart::Text(ref t) => {
                    string.0[last_idx] = TextStringPart::Text(t.trim_end().to_owned());
                }
            }
            Ok(string)
        }
    }
}

struct PartOfSpeechVisitor;

impl<'de> Visitor<'de> for PartOfSpeechVisitor {
    type Value = PartOfSpeech;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A part of speech value")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(PartOfSpeech {
            tag: value.to_owned(),
        })
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut tag = None;
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "tag" => {
                    tag = Some(map.next_value()?);
                }
                _ => {
                    return Err(serde::de::Error::unknown_field(
                        key.as_str(),
                        &["tag"],
                    ))
                }
            }
        }
        Ok(PartOfSpeech {
            tag: tag.ok_or_else(|| serde::de::Error::missing_field("tag"))?,
        })
    }
}

impl<'de> Deserialize<'de> for PartOfSpeech {
    fn deserialize<D>(deserializer: D) -> Result<PartOfSpeech, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(PartOfSpeechVisitor)
    }
}

struct DefinitionVisitor;

impl<'de> Visitor<'de> for DefinitionVisitor {
    type Value = Definition;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A definition value")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Definition {
            text: value.to_owned(),
            definition_type: Vec::new(),
        })
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut text = None;
        let mut definition_type = Vec::new();
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "text" => {
                    text = Some(map.next_value()?);
                }
                "$value" => {
                    text = Some(map.next_value()?);
                }
                "definitionType" => {
                    definition_type.push(map.next_value()?);
                }
                _ => {
                    return Err(serde::de::Error::unknown_field(
                        key.as_str(),
                        &["text", "definitionType"],
                    ))
                }
            }
        }
        Ok(Definition {
            text: text.ok_or_else(|| serde::de::Error::missing_field("text"))?,
            definition_type,
        })
    }
}

impl<'de> Deserialize<'de> for Definition {
    fn deserialize<D>(deserializer: D) -> Result<Definition, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(DefinitionVisitor)
    }
}

struct LabelVisitor;

impl<'de> Visitor<'de> for LabelVisitor {
    type Value = Label;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A label value")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Label {
            tag: value.to_owned(),
        })
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut tag = None;
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "tag" => {
                    tag = Some(map.next_value()?);
                }
                _ => {
                    return Err(serde::de::Error::unknown_field(
                        key.as_str(),
                        &["tag"],
                    ))
                }
            }
        }
        Ok(Label {
            tag: tag.ok_or_else(|| serde::de::Error::missing_field("tag"))?,
        })
    }
}

impl<'de> Deserialize<'de> for Label {
    fn deserialize<D>(deserializer: D) -> Result<Label, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(LabelVisitor)
    }
}

pub struct TranslationLanguageVisitor;

impl<'de> Visitor<'de> for TranslationLanguageVisitor {
    type Value = TranslationLanguage;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A translation language value")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(TranslationLanguage {
            lang_code: LangCode(value.to_owned()),
        })
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut lang_code = None;
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "langCode" => {
                    lang_code = Some(map.next_value()?);
                }
                _ => {
                    return Err(serde::de::Error::unknown_field(
                        key.as_str(),
                        &["tag", "script"],
                    ))
                }
            }
        }
        Ok(TranslationLanguage {
            lang_code: lang_code.ok_or_else(|| serde::de::Error::missing_field("tag"))?,
        })
    }
}

impl<'de> Deserialize<'de> for TranslationLanguage {
    fn deserialize<D>(deserializer: D) -> Result<TranslationLanguage, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(TranslationLanguageVisitor)
    }
}

pub struct ForPartOfSpeechVisitor;

impl<'de> Visitor<'de> for ForPartOfSpeechVisitor {
    type Value = ForPartOfSpeech;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A for part of speech value")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(ForPartOfSpeech {
            tag: value.to_owned(),
        })
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut tag = None;
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "tag" => {
                    tag = Some(map.next_value()?);
                }
                _ => {
                    return Err(serde::de::Error::unknown_field(
                        key.as_str(),
                        &["tag"],
                    ))
                }
            }
        }
        Ok(ForPartOfSpeech {
            tag: tag.ok_or_else(|| serde::de::Error::missing_field("tag"))?,
        })
    }
}

impl<'de> Deserialize<'de> for ForPartOfSpeech {
    fn deserialize<D>(deserializer: D) -> Result<ForPartOfSpeech, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ForPartOfSpeechVisitor)
    }
}

pub struct ForLanguageVisitor;

impl<'de> Visitor<'de> for ForLanguageVisitor {
    type Value = ForLanguage;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A for label value")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(ForLanguage {
            lang_code: LangCode(value.to_owned()),
        })
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut tag = None;
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "lang_code" => {
                    tag = Some(LangCode(map.next_value()?));
                }
                _ => {
                    return Err(serde::de::Error::unknown_field(
                        key.as_str(),
                        &["tag"],
                    ))
                }
            }
        }
        Ok(ForLanguage {
            lang_code: tag.ok_or_else(|| serde::de::Error::missing_field("tag"))?,
        })
    }
}

impl<'de> Deserialize<'de> for ForLanguage {
    fn deserialize<D>(deserializer: D) -> Result<ForLanguage, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ForLanguageVisitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::model::*;
    use serde_xml_rs;
    use std::fs::File;
    use serde::Deserialize;

    #[test]
    fn test_read_xml_0() {
        let file = File::open("examples/0.xml").unwrap();
        let _resource : LexicographicResource = serde_xml_rs::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_json_0() {
        let file = File::open("examples/0.json").unwrap();
        let _resource : LexicographicResource = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_1() {
        let file = File::open("examples/1.xml").unwrap();
        let _resource : Entry = serde_xml_rs::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_json_1() {
        let file = File::open("examples/1.json").unwrap();
        let _resource : Entry = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_2() {
        let file = File::open("examples/2.xml").unwrap();
        let _resource : Entry = serde_xml_rs::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_json_2() {
        let file = File::open("examples/2.json").unwrap();
        let _resource : Entry = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_3() {
        let file = File::open("examples/3.xml").unwrap();
        let _resource : Entry = serde_xml_rs::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_json_3() {
        let file = File::open("examples/3.json").unwrap();
        let _resource : Entry = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_4() {
        let file = File::open("examples/4.xml").unwrap();
        let _resource : Entry = serde_xml_rs::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_json_4() {
        let file = File::open("examples/4.json").unwrap();
        let _resource : Entry = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_5() {
        let file = File::open("examples/5.xml").unwrap();
        let _resource : LexicographicResource = serde_xml_rs::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_json_5() {
        let file = File::open("examples/5.json").unwrap();
        let _resource : LexicographicResource = serde_json::from_reader(file).unwrap();
    }

//    #[test]
//    fn test_read_xml_6() {
//        let file = File::open("examples/6.xml").unwrap();
//        let _resource : LexicographicResource = serde_xml_rs::from_reader(file).unwrap();
//    }
//
//    #[test]
//    fn test_read_json_6() {
//        let file = File::open("examples/6.json").unwrap();
//        let _resource : LexicographicResource = serde_json::from_reader(file).unwrap();
//    }

    #[test]
    fn test_read_xml_7() {
        let file = File::open("examples/7.xml").unwrap();
        let _resource : LexicographicResource = serde_xml_rs::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_json_7() {
        let file = File::open("examples/7.json").unwrap();
        let _resource : LexicographicResource = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_8() {
        let file = File::open("examples/8.xml").unwrap();
        let _resource : Entry = serde_xml_rs::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_json_8() {
        let file = File::open("examples/8.json").unwrap();
        let _resource : Entry = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_9() {
        let file = File::open("examples/9.xml").unwrap();
        let _resource : Entry = serde_xml_rs::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_json_9() {
        let file = File::open("examples/9.json").unwrap();
        let _resource : Entry = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_10() {
        let file = File::open("examples/10.xml").unwrap();
        let _resource : LexicographicResource = serde_xml_rs::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_json_10() {
        let file = File::open("examples/10.json").unwrap();
        let _resource : LexicographicResource = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_11() {
        let file = File::open("examples/11.xml").unwrap();
        let _resource : Entry = serde_xml_rs::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_json_11() {
        let file = File::open("examples/11.json").unwrap();
        let _resource : Entry = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_12() {
        let file = File::open("examples/12.xml").unwrap();
        let _resource : LexicographicResource = serde_xml_rs::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_json_12() {
        let file = File::open("examples/12.json").unwrap();
        let _resource : LexicographicResource = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_13() {
        let file = File::open("examples/13.xml").unwrap();
        let _resource : LexicographicResource = serde_xml_rs::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_json_13() {
        let file = File::open("examples/13.json").unwrap();
        let _resource : LexicographicResource = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_14() {
        let file = File::open("examples/14.xml").unwrap();
        let _resource : LexicographicResource = serde_xml_rs::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_json_14() {
        let file = File::open("examples/14.json").unwrap();
        let _resource : LexicographicResource = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_15() {
        let file = File::open("examples/15.xml").unwrap();
        let _resource : LexicographicResource = serde_xml_rs::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_json_15() {
        let file = File::open("examples/15.json").unwrap();
        let _resource : LexicographicResource = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_16() {
        let file = File::open("examples/16.xml").unwrap();
        let _resource : LexicographicResource = serde_xml_rs::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_json_16() {
        let file = File::open("examples/16.json").unwrap();
        let _resource : LexicographicResource = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_17() {
        let file = File::open("examples/17.xml").unwrap();
        let _resource : LexicographicResource = serde_xml_rs::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_json_17() {
        let file = File::open("examples/17.json").unwrap();
        let _resource : LexicographicResource = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_18() {
        let file = File::open("examples/18.xml").unwrap();
        let _resource : LexicographicResource = serde_xml_rs::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_json_18() {
        let file = File::open("examples/18.json").unwrap();
        let _resource : LexicographicResource = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_19() {
        let file = File::open("examples/19.xml").unwrap();
        let _resource : Entry = serde_xml_rs::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_json_19() {
        let file = File::open("examples/19.json").unwrap();
        let _resource : Entry = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_20() {
        let file = File::open("examples/20.xml").unwrap();
        let _resource : Entry = serde_xml_rs::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_json_20() {
        let file = File::open("examples/20.json").unwrap();
        let _resource : Entry = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_21() {
        let file = File::open("examples/21.xml").unwrap();
        let _resource : Entry = serde_xml_rs::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_json_21() {
        let file = File::open("examples/21.json").unwrap();
        let _resource : Entry = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_22() {
        let file = File::open("examples/22.xml").unwrap();
        let _resource : Entry = serde_xml_rs::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_json_22() {
        let file = File::open("examples/22.json").unwrap();
        let _resource : Entry = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_23() {
        let file = File::open("examples/23.xml").unwrap();
        let _resource : Entry = serde_xml_rs::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_json_23() {
        let file = File::open("examples/23.json").unwrap();
        let _resource : Entry = serde_json::from_reader(file).unwrap();
    }

    fn test_equivalent_lexicon(fname : &str) {
        let file1 = File::open(format!("examples/{}.xml", fname)).unwrap();
        let file2 = File::open(format!("examples/{}.json", fname)).unwrap();
        let mut resource1 : LexicographicResource = serde_xml_rs::from_reader(file1).unwrap();
        resource1.normalize();
        let resource2 : LexicographicResource = serde_json::from_reader(file2).unwrap();
        assert_eq!(resource1, resource2);
    }

    fn test_equivalent_entry(fname : &str) {
        let file1 = File::open(format!("examples/{}.xml", fname)).unwrap();
        let file2 = File::open(format!("examples/{}.json", fname)).unwrap();
        let mut deserializer = serde_xml_rs::de::Deserializer::new(
            serde_xml_rs::EventReader::new_with_config(file1, 
                serde_xml_rs::ParserConfig::new().trim_whitespace(false)));
        
        let mut resource1 : Entry = Entry::deserialize(&mut deserializer).unwrap();
        resource1.normalize();
        let resource2 : Entry = serde_json::from_reader(file2).unwrap();
        assert_eq!(resource1, resource2);
    }


    #[test]
    fn test_equivalent_0() {
        test_equivalent_lexicon("0");
    }

    #[test]
    fn test_equivalent_1() {
        test_equivalent_entry("1");
    }

    #[test]
    fn test_equivalent_2() {
        test_equivalent_entry("2");
    }

    #[test]
    fn test_equivalent_3() {
        test_equivalent_entry("3");
    }

    #[test]
    fn test_equivalent_4() {
        test_equivalent_entry("4");
    }

    #[test]
    fn test_equivalent_5() {
        test_equivalent_lexicon("5");
    }

//    #[test]
//    fn test_equivalent_6() {
//        test_equivalent_lexicon("6");
//    }

    #[test]
    fn test_equivalent_7() {
        test_equivalent_lexicon("7");
    }

    #[test]
    fn test_equivalent_8() {
        test_equivalent_entry("8");
    }

    #[test]
    fn test_equivalent_9() {
        test_equivalent_entry("9");
    }

    #[test]
    fn test_equivalent_10() {
        test_equivalent_lexicon("10");
    }

    #[test]
    fn test_equivalent_11() {
        test_equivalent_entry("11");
    }

    #[test]
    fn test_equivalent_12() {
        test_equivalent_lexicon("12");
    }

    #[test]
    fn test_equivalent_13() {
        test_equivalent_lexicon("13");
    }

    #[test]
    fn test_equivalent_14() {
        test_equivalent_lexicon("14");
    }

    #[test]
    fn test_equivalent_15() {
        test_equivalent_lexicon("15");
    }

    #[test]
    fn test_equivalent_16() {
        test_equivalent_lexicon("16");
    }

    #[test]
    fn test_equivalent_17() {
        test_equivalent_lexicon("17");
    }

    #[test]
    fn test_equivalent_18() {
        test_equivalent_lexicon("18");
    }

    #[test]
    fn test_equivalent_19() {
        test_equivalent_entry("19");
    }

    #[test]
    fn test_equivalent_20() {
        test_equivalent_entry("20");
    }

    #[test]
    fn test_equivalent_21() {
        test_equivalent_entry("21");
    }

    #[test]
    fn test_equivalent_22() {
        test_equivalent_entry("22");
    }

    #[test]
    fn test_equivalent_23() {
        test_equivalent_entry("23");
    }
}
