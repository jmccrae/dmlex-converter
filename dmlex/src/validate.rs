/// This module evaluates the uniqueness constraints in the model.
use crate::model::*;

pub trait Validate<S: PartialEq + FragId> {
    fn check_uniqueness(&self) -> Vec<String>;
    fn signature(&self) -> S;

    fn frag_path(&self) -> String {
        self.signature().frag_id()
    }
}

pub trait FragId {
    fn frag_id(&self) -> String;
}

fn all_unique<S: PartialEq>(items: &Vec<S>) -> bool {
    for i in 0..items.len() {
        for j in i+1..items.len() {
            if items[i] == items[j] {
                return false;
            }
        }
    }
    true
}

macro_rules! check_rec {
    ($self:ident, $field:ident, $errors:ident) => {
        let mut signatures = Vec::new();
        for elem in &$self.$field {
            $errors.extend(elem.check_uniqueness());
            signatures.push(elem.signature());
        }
        if !all_unique(&signatures) {
            $errors.push(format!("Duplicate elements in {}", stringify!($field)));
        }
    }
}

impl Validate<String> for LexicographicResource {
    fn check_uniqueness(&self) -> Vec<String> {
        let mut errors = Vec::new();

        check_rec!(self, entries, errors);
        check_rec!(self, definition_type_tags, errors);
        check_rec!(self, inflected_form_tags, errors);
        check_rec!(self, label_tags, errors);
        check_rec!(self, label_type_tags, errors);
        check_rec!(self, part_of_speech_tags, errors);
        check_rec!(self, source_identity_tags, errors);
        check_rec!(self, transcription_scheme_tags, errors);
        for elem in &self.relations { // Relations have no uniqueness constraints
            errors.extend(elem.check_uniqueness());
        }
        check_rec!(self, relation_types, errors);
        check_rec!(self, etymon_languages, errors);
        check_rec!(self, etymon_types, errors);


        return errors;
    }

    fn signature(&self) -> String {
        panic!("Lexicographic Resource does not have a signature");
    }

    fn frag_path(&self) -> String {
        if let Some(uri) = &self.uri {
            uri.clone()
        } else {
            "".to_string()
        }
    }
}

impl Validate<(String, Option<u32>, Vec<String>)> for Entry {
    fn check_uniqueness(&self) -> Vec<String> {
        let mut errors = Vec::new();

        check_rec!(self, pronunciations, errors);
        check_rec!(self, inflected_forms, errors);
        check_rec!(self, senses, errors);
        check_rec!(self, etymologies, errors);

        if !all_unique(&self.labels) {
            errors.push("Duplicate labels".to_string());
        }

        return errors;
    }

    fn signature(&self) -> (String, Option<u32>, Vec<String>) {
        (self.headword.clone(), 
         self.homograph_number.clone(), 
         self.parts_of_speech.clone())
    }
}

impl Validate<(String, Option<String>)> for InflectedForm {
    fn check_uniqueness(&self) -> Vec<String> {
        let mut errors = Vec::new();

        check_rec!(self, pronunciations, errors);

        if !all_unique(&self.labels) {
            errors.push("Duplicate labels".to_string());
        }

        return errors;
    }

    fn signature(&self) -> (String, Option<String>) {
        (self.text.clone(), self.tag.clone())
    }
}

impl Validate<(Option<String>, Vec<Definition>)> for Sense {
    fn check_uniqueness(&self) -> Vec<String> {
        let mut errors = Vec::new();

        check_rec!(self, examples, errors);
        check_rec!(self, definitions, errors);
        check_rec!(self, headword_explanations, errors);
        check_rec!(self, headword_translations, errors);

        if !all_unique(&self.labels) {
            errors.push("Duplicate labels".to_string());
        }

        return errors;
    }

    fn signature(&self) -> (Option<String>, Vec<Definition>) {
        (self.indicator.clone(), self.definitions.clone())
    }
}

impl Validate<String> for Definition {
    fn check_uniqueness(&self) -> Vec<String> {
        Vec::new()
    }

    fn signature(&self) -> String {
        self.text.clone()
    }
}

impl FragId for Definition {
    fn frag_id(&self) -> String {
        self.text.frag_id()
    }
}

impl Validate<(Option<String>, Vec<Transcription>)> for Pronunciation {
    fn check_uniqueness(&self) -> Vec<String> {
        let mut errors = Vec::new();

        check_rec!(self, transcriptions, errors);

        return errors;
    }

    fn signature(&self) -> (Option<String>, Vec<Transcription>) {
        (self.sound_file.clone(), self.transcriptions.clone())
    }
}

impl Validate<String> for Transcription {
    fn check_uniqueness(&self) -> Vec<String> {
        Vec::new()
    }

    fn signature(&self) -> String {
        self.text.clone()
    }
}

impl FragId for Transcription {
    fn frag_id(&self) -> String {
        self.text.frag_id()
    }
}

impl Validate<String> for Example {
    fn check_uniqueness(&self) -> Vec<String> {
        let mut errors = Vec::new();

        check_rec!(self, example_translations, errors);

        if !all_unique(&self.labels) {
            errors.push("Duplicate labels".to_string());
        }

        return errors;
    }

    fn signature(&self) -> String {
        self.text.clone()
    }
}

impl Validate<String> for LangCode {
    fn check_uniqueness(&self) -> Vec<String> {
        Vec::new()
    }

    fn signature(&self) -> String {
        self.0.clone()
    }
}

impl Validate<(String, Option<LangCode>)> for HeadwordTranslation {
    fn check_uniqueness(&self) -> Vec<String> {
        let mut errors = Vec::new();

        check_rec!(self, pronunciations, errors);
        check_rec!(self, inflected_forms, errors);

        if !all_unique(&self.labels) {
            errors.push("Duplicate labels".to_string());
        }

        return errors;
    }

    fn signature(&self) -> (String, Option<LangCode>) {
        (self.text.clone(), self.lang_code.clone())
    }
}


impl Validate<(String, Option<LangCode>)> for HeadwordExplanation {
    fn check_uniqueness(&self) -> Vec<String> {
        Vec::new()
    }

    fn signature(&self) -> (String, Option<LangCode>) {
        (self.text.clone(), self.lang_code.clone())
    }
}

impl Validate<(String, Option<LangCode>)> for ExampleTranslation {
    fn check_uniqueness(&self) -> Vec<String> {
        Vec::new()
    }

    fn signature(&self) -> (String, Option<LangCode>) {
        (self.text.clone(), self.lang_code.clone())
    }
}

impl Validate<String> for DefinitionTypeTag {
    fn check_uniqueness(&self) -> Vec<String> {
        if all_unique(&self.same_as) {
            Vec::new()
        } else {
            vec!["Duplicate same_as".to_string()]
        }

    }

    fn signature(&self) -> String {
        self.tag.clone()
    }
}

impl Validate<String> for InflectedFormTag {
    fn check_uniqueness(&self) -> Vec<String> {
        if all_unique(&self.same_as) {
            Vec::new()
        } else {
            vec!["Duplicate same_as".to_string()]
        }
    }

    fn signature(&self) -> String {
        self.tag.clone()
    }
}

impl Validate<String> for LabelTag {
    fn check_uniqueness(&self) -> Vec<String> {
        if all_unique(&self.same_as) {
            Vec::new()
        } else {
            vec!["Duplicate same_as".to_string()]
        }
    }

    fn signature(&self) -> String {
        self.tag.clone()
    }
}

impl Validate<String> for LabelTypeTag {
    fn check_uniqueness(&self) -> Vec<String> {
        if all_unique(&self.same_as) {
            Vec::new()
        } else {
            vec!["Duplicate same_as".to_string()]
        }
    }

    fn signature(&self) -> String {
        self.tag.clone()
    }
}

impl Validate<String> for PartOfSpeechTag {
    fn check_uniqueness(&self) -> Vec<String> {
        if all_unique(&self.same_as) {
            Vec::new()
        } else {
            vec!["Duplicate same_as".to_string()]
        }
    }

    fn signature(&self) -> String {
        self.tag.clone()
    }
}

impl Validate<String> for SourceIdentityTag {
    fn check_uniqueness(&self) -> Vec<String> {
        if all_unique(&self.same_as) {
            Vec::new()
        } else {
            vec!["Duplicate same_as".to_string()]
        }
    }

    fn signature(&self) -> String {
        self.tag.clone()
    }
}

impl Validate<String> for TranscriptionSchemeTag {
    fn check_uniqueness(&self) -> Vec<String> {
        Vec::new()
    }

    fn signature(&self) -> String {
        self.tag.clone()
    }
}

impl Validate<String> for Relation {
    fn check_uniqueness(&self) -> Vec<String> {
        let mut errors = Vec::new();

        check_rec!(self, members, errors);

        Vec::new()
    }

    fn signature(&self) -> String {
        panic!("Relation does not have a signature");
    }
}

impl Validate<(String, Option<String>)> for Member {
    fn check_uniqueness(&self) -> Vec<String> {
        Vec::new()
    }

    fn signature(&self) -> (String, Option<String>) {
        (self.ref_.clone(), self.role.clone())
    }
}

impl Validate<String> for RelationType {
    fn check_uniqueness(&self) -> Vec<String> {
        let mut errors = Vec::new();

        check_rec!(self, member_types, errors);

        if !all_unique(&self.same_as) {
            errors.push("Duplicate same_as".to_string());
        }
        return errors;
    }

    fn signature(&self) -> String {
        self._type.clone()
    }
}

impl Validate<(Option<String>, MemberTypeType)> for MemberType {
    fn check_uniqueness(&self) -> Vec<String> {
        Vec::new()
    }

    fn signature(&self) -> (Option<String>, MemberTypeType) {
        (self.role.clone(), self._type.clone())
    }
}

impl FragId for MemberTypeType {
    fn frag_id(&self) -> String {
        match self {
            MemberTypeType::Sense => "sense".to_string(),
            MemberTypeType::Entry => "entry".to_string(),
            MemberTypeType::Collocate => "collocate".to_string(),
        }
    }
}

impl Validate<(Option<String>, Vec<Etymon>)> for Etymology {
    fn check_uniqueness(&self) -> Vec<String> {
        let mut errors = Vec::new();

        check_rec!(self, etymons, errors);

        return errors;
    }

    fn signature(&self) -> (Option<String>, Vec<Etymon>) {
        (self.description.clone(), self.etymons.clone())
    }
}

impl Validate<(Option<String>, Vec<EtymonUnit>)> for Etymon {
    fn check_uniqueness(&self) -> Vec<String> {
        let mut errors = Vec::new();

        check_rec!(self, etymon_units, errors);

        return errors;
    }

    fn signature(&self) -> (Option<String>, Vec<EtymonUnit>) {
        (self.when.clone(), self.etymon_units.clone())
    }
}

impl FragId for Etymon {
    fn frag_id(&self) -> String {
        self.when.frag_id() + "~" + &self.etymon_units.frag_id()
    }
}

impl Validate<(String, LangCode)> for EtymonUnit {
    fn check_uniqueness(&self) -> Vec<String> {
        Vec::new()
    }

    fn signature(&self) -> (String, LangCode) {
        (self.text.clone(), self.lang_code.clone())
    }
}

impl FragId for EtymonUnit {
    fn frag_id(&self) -> String {
        self.lang_code.frag_id() + "~" + &self.text.frag_id()
    }
}

impl Validate<String> for EtymonType {
    fn check_uniqueness(&self) -> Vec<String> {
        Vec::new()
    }

    fn signature(&self) -> String {
        self._type.clone()
    }
}

impl Validate<LangCode> for EtymonLanguage {
    fn check_uniqueness(&self) -> Vec<String> {
        Vec::new()
    }

    fn signature(&self) -> LangCode {
        self.lang_code.clone()
    }
}

fn percentage_encode(input: &str) -> String {
    let mut result = String::new();
    for byte in input.bytes() {
        match byte {
            // Unreserved characters
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(byte as char);
            }
            // Reserved characters
            b'!' | b'<' | b'>' | b'*' | b'\'' | b'(' | b')' | b';' | b':' | b'@' | b'&' | b'=' | b'+' | b'$' | b',' | b'/' | b'?' | b'#' | b'[' | b']' | b' ' | b'"' | b'%' | b'{' | b'}' | b'|' | b'\\' | b'^' | b'`' => {
                result.push('%');
                result.push_str(&format!("{:02X}", byte));
            }
            // Other characters
            _ => {
                result.push('%');
                result.push_str(&format!("{:02X}", byte));
            }
        }
    }
    result
}


impl FragId for String {
    fn frag_id(&self) -> String {
        percentage_encode(&self.replace("\\","\\\\")
            .replace("~","\\~")
            .replace("_", "\\_"))
    }
}

impl<A : FragId, B: FragId> FragId for (A, B) {
    fn frag_id(&self) -> String {
        let s1 = self.0.frag_id();
        let s2 = self.1.frag_id();
        if s1.is_empty() {
            s2
        } else if s2.is_empty() {
            s1
        } else {
            format!("{}~{}", s1, s2)
        }
    }
}

impl<A : FragId, B : FragId, C : FragId> FragId for (A, B, C) {
    fn frag_id(&self) -> String {
        let s1 = self.0.frag_id();
        let s2 = self.1.frag_id();
        let s3 = self.2.frag_id();
        if s1.is_empty() {
            if s2.is_empty() {
                s3
            } else if s3.is_empty() {
                s2
            } else {
                format!("{}~{}", s2, s3)
            }
        } else if s2.is_empty() {
            if s3.is_empty() {
                s1
            } else {
                format!("{}~{}", s1, s3)
            }
        } else {
            format!("{}~{}~{}", s1, s2, s3)
        }
    }
}

impl FragId for LangCode {
    fn frag_id(&self) -> String {
        self.0.frag_id()
    }
}

impl<A : FragId> FragId for Option<A> {
    fn frag_id(&self) -> String {
        match self {
            Some(a) => a.frag_id(),
            None => "".to_string()
        }
    }
}

impl<A : FragId> FragId for Vec<A> {
    fn frag_id(&self) -> String {
        let mut result = String::new();
        for a in self {
            if !result.is_empty() {
                result.push('_');
            }
            result.push_str(&a.frag_id());
        }
        result
    }
}

impl FragId for u32 {
    fn frag_id(&self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::model::*;
    use std::fs::File;
    use crate::validate::Validate;

    #[test]
    fn test_validate_xml_0() {
        let file = File::open("examples/0.xml").unwrap();
        let resource : LexicographicResource = crate::read_xml::read_xml(file, "lexicographicResource").unwrap();
        resource.validate().unwrap();
    }

    #[test]
    fn test_validate_xml_1() {
        let file = File::open("examples/1.xml").unwrap();
        let resource : Entry = crate::read_xml::read_xml(file, "entry").unwrap();
        resource.validate().unwrap();
    }

    #[test]
    fn test_validate_xml_2() {
        let file = File::open("examples/2.xml").unwrap();
        let resource : Entry = crate::read_xml::read_xml(file, "entry").unwrap();
        resource.validate().unwrap();
    }

    #[test]
    fn test_validate_xml_3() {
        let file = File::open("examples/3.xml").unwrap();
        let resource : Entry = crate::read_xml::read_xml(file, "entry").unwrap();
        resource.validate().unwrap();
    }

    #[test]
    fn test_validate_xml_4() {
        let file = File::open("examples/4.xml").unwrap();
        let resource : Entry = crate::read_xml::read_xml(file, "entry").unwrap();
        resource.validate().unwrap();
    }

    #[test]
    fn test_validate_xml_5() {
        let file = File::open("examples/5.xml").unwrap();
        let resource : LexicographicResource = crate::read_xml::read_xml(file, "lexicographicResource").unwrap();
        resource.validate().unwrap();
    }

    #[test]
    fn test_validate_xml_6() {
        let file = File::open("examples/6.xml").unwrap();
        let resource : LexicographicResource = crate::read_xml::read_xml(file, "lexicographicResource").unwrap();
        resource.validate().unwrap();
    }

    #[test]
    fn test_validate_xml_7() {
        let file = File::open("examples/7.xml").unwrap();
        let resource : LexicographicResource = crate::read_xml::read_xml(file, "lexicographicResource").unwrap();
        resource.validate().unwrap();
    }

    #[test]
    fn test_validate_xml_8() {
        let file = File::open("examples/8.xml").unwrap();
        let resource : Entry = crate::read_xml::read_xml(file, "entry").unwrap();
        resource.validate().unwrap();
    }

    #[test]
    fn test_validate_xml_9() {
        let file = File::open("examples/9.xml").unwrap();
        let resource : Entry = crate::read_xml::read_xml(file, "entry").unwrap();
        resource.validate().unwrap();
    }

    #[test]
    fn test_validate_xml_10() {
        let file = File::open("examples/10.xml").unwrap();
        let resource : LexicographicResource = crate::read_xml::read_xml(file, "lexicographicResource").unwrap();
        resource.validate().unwrap();
    }

//    #[test]
//    fn test_validate_xml_11() {
//        let file = File::open("examples/11.xml").unwrap();
//        let resource : Entry = crate::read_xml::read_xml(file, "entry").unwrap();
//        resource.validate().unwrap();
//    }

    #[test]
    fn test_validate_xml_12() {
        let file = File::open("examples/12.xml").unwrap();
        let resource : LexicographicResource = crate::read_xml::read_xml(file, "lexicographicResource").unwrap();
        resource.validate().unwrap();
    }

    #[test]
    fn test_validate_xml_13() {
        let file = File::open("examples/13.xml").unwrap();
        let resource : LexicographicResource = crate::read_xml::read_xml(file, "lexicographicResource").unwrap();
        resource.validate().unwrap();
    }

    #[test]
    fn test_validate_xml_14() {
        let file = File::open("examples/14.xml").unwrap();
        let resource : LexicographicResource = crate::read_xml::read_xml(file, "lexicographicResource").unwrap();
        resource.validate().unwrap();
    }

    #[test]
    fn test_validate_xml_15() {
        let file = File::open("examples/15.xml").unwrap();
        let resource : LexicographicResource = crate::read_xml::read_xml(file, "lexicographicResource").unwrap();
        resource.validate().unwrap();
    }

    #[test]
    fn test_validate_xml_16() {
        let file = File::open("examples/16.xml").unwrap();
        let resource : LexicographicResource = crate::read_xml::read_xml(file, "lexicographicResource").unwrap();
        resource.validate().unwrap();
    }

    #[test]
    fn test_validate_xml_17() {
        let file = File::open("examples/17.xml").unwrap();
        let resource : LexicographicResource = crate::read_xml::read_xml(file, "lexicographicResource").unwrap();
        resource.validate().unwrap();
    }

    #[test]
    fn test_validate_xml_18() {
        let file = File::open("examples/18.xml").unwrap();
        let resource : LexicographicResource = crate::read_xml::read_xml(file, "lexicographicResource").unwrap();
        resource.validate().unwrap();
    }

    #[test]
    fn test_validate_xml_19() {
        let file = File::open("examples/19.xml").unwrap();
        let resource : Entry = crate::read_xml::read_xml(file, "entry").unwrap();
        resource.validate().unwrap();
    }

    #[test]
    fn test_validate_xml_20() {
        let file = File::open("examples/20.xml").unwrap();
        let resource : Entry = crate::read_xml::read_xml(file, "entry").unwrap();
        resource.validate().unwrap();
    }

    #[test]
    fn test_validate_xml_21() {
        let file = File::open("examples/21.xml").unwrap();
        let resource : Entry = crate::read_xml::read_xml(file, "entry").unwrap();
        resource.validate().unwrap();
    }

    #[test]
    fn test_validate_xml_22() {
        let file = File::open("examples/22.xml").unwrap();
        let resource : Entry = crate::read_xml::read_xml(file, "entry").unwrap();
        resource.validate().unwrap();
    }

    #[test]
    fn test_validate_xml_23() {
        let file = File::open("examples/23.xml").unwrap();
        let resource : Entry = crate::read_xml::read_xml(file, "entry").unwrap();
        resource.validate().unwrap();
    }

    #[test]
    fn test_validate_xml_24() {
        let file = File::open("examples/24.xml").unwrap();
        let resource : LexicographicResource = crate::read_xml::read_xml(file, "lexicographicResource").unwrap();
        resource.validate().unwrap();
    }

    #[test]
    fn test_frag_id() {
        let mut resource = LexicographicResource::default();
        resource.title = Some("Test".to_string());
        resource.uri = Some("http://example.com/lexicon".to_string());
        resource.lang_code = LangCode("en".to_string());
        
        let mut entry = Entry::default();
        entry.headword = "test".to_string();
        let mut sense = Sense::default();
        let mut definition = Definition::default();
        definition.text = "test definition".to_string();
        sense.definitions.push(definition);

        assert_eq!(entry.frag_path(), "test");
        assert_eq!(sense.frag_path(), "test%20definition");
        
        entry.senses.push(sense);
        resource.entries.push(entry);

        assert_eq!(resource.frag_path(), "http://example.com/lexicon");
    }

}
 
