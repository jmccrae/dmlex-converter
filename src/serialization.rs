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
                        &["placeholderMarker", "$value"],
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
        deserializer.deserialize_any(HeadwordStringVisitor)
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
                    content.push(TextStringPart::CollocateMarker(map.next_value()?));
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
                        &["placeholderMarker", "$value"],
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
        deserializer.deserialize_any(TextStringVisitor)
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

#[cfg(test)]
mod tests {
    use crate::model::*;
    use serde_xml_rs;
    use std::fs::File;

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
}
