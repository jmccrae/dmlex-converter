#[cfg(test)]
mod tests {
    use crate::model::*;
    use std::fs::File;
    use crate::write_xml::WriteXML;

    #[test]
    fn test_read_xml_0() {
        let file = File::open("examples/0.xml").unwrap();
        let _resource : LexicographicResource = crate::read_xml::read_xml(file, "lexicographicResource").unwrap();
    }

    #[test]
    fn test_read_json_0() {
        let file = File::open("examples/0.json").unwrap();
        let _resource : LexicographicResource = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_1() {
        let file = File::open("examples/1.xml").unwrap();
        let _resource : Entry = crate::read_xml::read_xml(file, "entry").unwrap();
    }

    #[test]
    fn test_read_json_1() {
        let file = File::open("examples/1.json").unwrap();
        let _resource : Entry = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_2() {
        let file = File::open("examples/2.xml").unwrap();
        let _resource : Entry = crate::read_xml::read_xml(file, "entry").unwrap();
    }

    #[test]
    fn test_read_json_2() {
        let file = File::open("examples/2.json").unwrap();
        let _resource : Entry = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_3() {
        let file = File::open("examples/3.xml").unwrap();
        let _resource : Entry = crate::read_xml::read_xml(file, "entry").unwrap();
    }

    #[test]
    fn test_read_json_3() {
        let file = File::open("examples/3.json").unwrap();
        let _resource : Entry = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_4() {
        let file = File::open("examples/4.xml").unwrap();
        let _resource : Entry = crate::read_xml::read_xml(file, "entry").unwrap();
    }

    #[test]
    fn test_read_json_4() {
        let file = File::open("examples/4.json").unwrap();
        let _resource : Entry = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_5() {
        let file = File::open("examples/5.xml").unwrap();
        let _resource : LexicographicResource = crate::read_xml::read_xml(file, "lexicographicResource").unwrap();
    }

    #[test]
    fn test_read_json_5() {
        let file = File::open("examples/5.json").unwrap();
        let _resource : LexicographicResource = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_6() {
        let file = File::open("examples/6.xml").unwrap();
        let _resource : LexicographicResource = crate::read_xml::read_xml(file, "lexicographicResource").unwrap();
    }

    #[test]
    fn test_read_json_6() {
        let file = File::open("examples/6.json").unwrap();
        let _resource : LexicographicResource = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_7() {
        let file = File::open("examples/7.xml").unwrap();
        let _resource : LexicographicResource = crate::read_xml::read_xml(file, "lexicographicResource").unwrap();
    }

    #[test]
    fn test_read_json_7() {
        let file = File::open("examples/7.json").unwrap();
        let _resource : LexicographicResource = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_8() {
        let file = File::open("examples/8.xml").unwrap();
        let _resource : Entry = crate::read_xml::read_xml(file, "entry").unwrap();
    }

    #[test]
    fn test_read_json_8() {
        let file = File::open("examples/8.json").unwrap();
        let _resource : Entry = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_9() {
        let file = File::open("examples/9.xml").unwrap();
        let _resource : Entry = crate::read_xml::read_xml(file, "entry").unwrap();
    }

    #[test]
    fn test_read_json_9() {
        let file = File::open("examples/9.json").unwrap();
        let _resource : Entry = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_10() {
        let file = File::open("examples/10.xml").unwrap();
        let _resource : LexicographicResource = crate::read_xml::read_xml(file, "lexicographicResource").unwrap();
    }

    #[test]
    fn test_read_json_10() {
        let file = File::open("examples/10.json").unwrap();
        let _resource : LexicographicResource = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_11() {
        let file = File::open("examples/11.xml").unwrap();
        let _resource : Entry = crate::read_xml::read_xml(file, "entry").unwrap();
    }

    #[test]
    fn test_read_json_11() {
        let file = File::open("examples/11.json").unwrap();
        let _resource : Entry = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_12() {
        let file = File::open("examples/12.xml").unwrap();
        let _resource : LexicographicResource = crate::read_xml::read_xml(file, "lexicographicResource").unwrap();
    }

    #[test]
    fn test_read_json_12() {
        let file = File::open("examples/12.json").unwrap();
        let _resource : LexicographicResource = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_13() {
        let file = File::open("examples/13.xml").unwrap();
        let _resource : LexicographicResource = crate::read_xml::read_xml(file, "lexicographicResource").unwrap();
    }

    #[test]
    fn test_read_json_13() {
        let file = File::open("examples/13.json").unwrap();
        let _resource : LexicographicResource = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_14() {
        let file = File::open("examples/14.xml").unwrap();
        let _resource : LexicographicResource = crate::read_xml::read_xml(file, "lexicographicResource").unwrap();
    }

    #[test]
    fn test_read_json_14() {
        let file = File::open("examples/14.json").unwrap();
        let _resource : LexicographicResource = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_15() {
        let file = File::open("examples/15.xml").unwrap();
        let _resource : LexicographicResource = crate::read_xml::read_xml(file, "lexicographicResource").unwrap();
    }

    #[test]
    fn test_read_json_15() {
        let file = File::open("examples/15.json").unwrap();
        let _resource : LexicographicResource = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_16() {
        let file = File::open("examples/16.xml").unwrap();
        let _resource : LexicographicResource = crate::read_xml::read_xml(file, "lexicographicResource").unwrap();
    }

    #[test]
    fn test_read_json_16() {
        let file = File::open("examples/16.json").unwrap();
        let _resource : LexicographicResource = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_17() {
        let file = File::open("examples/17.xml").unwrap();
        let _resource : LexicographicResource = crate::read_xml::read_xml(file, "lexicographicResource").unwrap();
    }

    #[test]
    fn test_read_json_17() {
        let file = File::open("examples/17.json").unwrap();
        let _resource : LexicographicResource = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_18() {
        let file = File::open("examples/18.xml").unwrap();
        let _resource : LexicographicResource = crate::read_xml::read_xml(file, "lexicographicResource").unwrap();
    }

    #[test]
    fn test_read_json_18() {
        let file = File::open("examples/18.json").unwrap();
        let _resource : LexicographicResource = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_19() {
        let file = File::open("examples/19.xml").unwrap();
        let _resource : Entry = crate::read_xml::read_xml(file, "entry").unwrap();
    }

    #[test]
    fn test_read_json_19() {
        let file = File::open("examples/19.json").unwrap();
        let _resource : Entry = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_20() {
        let file = File::open("examples/20.xml").unwrap();
        let _resource : Entry = crate::read_xml::read_xml(file, "entry").unwrap();
    }

    #[test]
    fn test_read_json_20() {
        let file = File::open("examples/20.json").unwrap();
        let _resource : Entry = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_21() {
        let file = File::open("examples/21.xml").unwrap();
        let _resource : Entry = crate::read_xml::read_xml(file, "entry").unwrap();
    }

    #[test]
    fn test_read_json_21() {
        let file = File::open("examples/21.json").unwrap();
        let _resource : Entry = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_22() {
        let file = File::open("examples/22.xml").unwrap();
        let _resource : Entry = crate::read_xml::read_xml(file, "entry").unwrap();
    }

    #[test]
    fn test_read_json_22() {
        let file = File::open("examples/22.json").unwrap();
        let _resource : Entry = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_23() {
        let file = File::open("examples/23.xml").unwrap();
        let _resource : Entry = crate::read_xml::read_xml(file, "entry").unwrap();
    }

    #[test]
    fn test_read_json_23() {
        let file = File::open("examples/23.json").unwrap();
        let _resource : Entry = serde_json::from_reader(file).unwrap();
    }

    #[test]
    fn test_read_xml_24() {
        let file = File::open("examples/24.xml").unwrap();
        let _resource : LexicographicResource = crate::read_xml::read_xml(file, "lexicographicResource").unwrap();
    }

    #[test]
    fn test_read_json_24() {
        let file = File::open("examples/24.json").unwrap();
        let _resource : LexicographicResource = serde_json::from_reader(file).unwrap();
    }

 
    fn test_equivalent_lexicon(fname : &str) {
        let file1 = File::open(format!("examples/{}.xml", fname)).unwrap();
        let file2 = File::open(format!("examples/{}.json", fname)).unwrap();
        let resource1 : LexicographicResource = crate::read_xml::read_xml(file1, "lexicographicResource").unwrap();
        let resource2 : LexicographicResource = serde_json::from_reader(file2).unwrap();
        assert_eq!(resource1, resource2);
    }

    fn test_equivalent_entry(fname : &str) {
        let file1 = File::open(format!("examples/{}.xml", fname)).unwrap();
        let file2 = File::open(format!("examples/{}.json", fname)).unwrap();
        let resource1 : Entry = crate::read_xml::read_xml(file1, "entry").unwrap();
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

    #[test]
    fn test_equivalent_6() {
        test_equivalent_lexicon("6");
    }

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

    #[test]
    fn test_equivalent_24() {
        test_equivalent_lexicon("24");
    }


    fn json_round_trip_lexicon(fname : &str) {
        let file = File::open(format!("examples/{}.json", fname)).unwrap();
        let resource1 : LexicographicResource = serde_json::from_reader(file).unwrap();
        let to_str = serde_json::to_string(&resource1).unwrap();
        let resource2 : LexicographicResource = serde_json::from_str(&to_str).unwrap();
        assert_eq!(resource1, resource2);
    }

    fn json_round_trip_entry(fname : &str) {
        let file = File::open(format!("examples/{}.json", fname)).unwrap();
        let resource1 : Entry = serde_json::from_reader(file).unwrap();
        let to_str = serde_json::to_string(&resource1).unwrap();
        let resource2 : Entry = serde_json::from_str(&to_str).unwrap();
        assert_eq!(resource1, resource2);
    }


    #[test]
    fn test_json_round_trip_0() {
        json_round_trip_lexicon("0");
    }

    #[test]
    fn test_json_round_trip_1() {
        json_round_trip_entry("1");
    }

    #[test]
    fn test_json_round_trip_2() {
        json_round_trip_entry("2");
    }

    #[test]
    fn test_json_round_trip_3() {
        json_round_trip_entry("3");
    }

    #[test]
    fn test_json_round_trip_4() {
        json_round_trip_entry("4");
    }

    #[test]
    fn test_json_round_trip_5() {
        json_round_trip_lexicon("5");
    }

    #[test]
    fn test_json_round_trip_6() {
        json_round_trip_lexicon("6");
    }

    #[test]
    fn test_json_round_trip_7() {
        json_round_trip_lexicon("7");
    }

    #[test]
    fn test_json_round_trip_8() {
        json_round_trip_entry("8");
    }

    #[test]
    fn test_json_round_trip_9() {
        json_round_trip_entry("9");
    }

    #[test]
    fn test_json_round_trip_10() {
        json_round_trip_lexicon("10");
    }

    #[test]
    fn test_json_round_trip_11() {
        json_round_trip_entry("11");
    }

    #[test]
    fn test_json_round_trip_12() {
        json_round_trip_lexicon("12");
    }

    #[test]
    fn test_json_round_trip_13() {
        json_round_trip_lexicon("13");
    }

    #[test]
    fn test_json_round_trip_14() {
        json_round_trip_lexicon("14");
    }

    #[test]
    fn test_json_round_trip_15() {
        json_round_trip_lexicon("15");
    }

    #[test]
    fn test_json_round_trip_16() {
        json_round_trip_lexicon("16");
    }

    #[test]
    fn test_json_round_trip_17() {
        json_round_trip_lexicon("17");
    }

    #[test]
    fn test_json_round_trip_18() {
        json_round_trip_lexicon("18");
    }

    #[test]
    fn test_json_round_trip_19() {
        json_round_trip_entry("19");
    }

    #[test]
    fn test_json_round_trip_20() {
        json_round_trip_entry("20");
    }

    #[test]
    fn test_json_round_trip_21() {
        json_round_trip_entry("21");
    }

    #[test]
    fn test_json_round_trip_22() {
        json_round_trip_entry("22");
    }

    #[test]
    fn test_json_round_trip_23() {
        json_round_trip_entry("23");
    }

    #[test]
    fn test_json_round_trip_24() {
        json_round_trip_lexicon("24");
    }


    fn xml_round_trip_lexicon(fname: &str) {
        let file1 = File::open(format!("examples/{}.xml", fname)).unwrap();
        let resource1 : LexicographicResource = crate::read_xml::read_xml(file1, "lexicographicResource").unwrap();
        let mut out = Vec::new();
        let mut writer = xml::EmitterConfig::new().create_writer(&mut out);
        (&resource1).write_xml(&mut writer).unwrap();
        let resource2 : LexicographicResource = crate::read_xml::read_xml(&out[..], "lexicographicResource").unwrap();
        assert_eq!(resource1, resource2);
    }

    fn xml_round_trip_entry(fname: &str) {
        let file1 = File::open(format!("examples/{}.xml", fname)).unwrap();
        let resource1 : Entry = crate::read_xml::read_xml(file1, "entry").unwrap();
        let mut out = Vec::new();
        let mut writer = xml::EmitterConfig::new().create_writer(&mut out);
        (&resource1).write_xml(&mut writer).unwrap();
        let resource2 : Entry = crate::read_xml::read_xml(&out[..], "entry").unwrap();
        assert_eq!(resource1, resource2);
    }

    #[test]
    fn test_xml_round_trip_0() {
        xml_round_trip_lexicon("0");
    }

    #[test]
    fn test_xml_round_trip_1() {
        xml_round_trip_entry("1");
    }

    #[test]
    fn test_xml_round_trip_2() {
        xml_round_trip_entry("2");
    }

    #[test]
    fn test_xml_round_trip_3() {
        xml_round_trip_entry("3");
    }

    #[test]
    fn test_xml_round_trip_4() {
        xml_round_trip_entry("4");
    }

    #[test]
    fn test_xml_round_trip_5() {
        xml_round_trip_lexicon("5");
    }

    #[test]
    fn test_xml_round_trip_6() {
        xml_round_trip_lexicon("6");
    }

    #[test]
    fn test_xml_round_trip_7() {
        xml_round_trip_lexicon("7");
    }

    #[test]
    fn test_xml_round_trip_8() {
        xml_round_trip_entry("8");
    }

    #[test]
    fn test_xml_round_trip_9() {
        xml_round_trip_entry("9");
    }

    #[test]
    fn test_xml_round_trip_10() {
        xml_round_trip_lexicon("10");
    }

    #[test]
    fn test_xml_round_trip_11() {
        xml_round_trip_entry("11");
    }

    #[test]
    fn test_xml_round_trip_12() {
        xml_round_trip_lexicon("12");
    }

    #[test]
    fn test_xml_round_trip_13() {
        xml_round_trip_lexicon("13");
    }

    #[test]
    fn test_xml_round_trip_14() {
        xml_round_trip_lexicon("14");
    }

    #[test]
    fn test_xml_round_trip_15() {
        xml_round_trip_lexicon("15");
    }

    #[test]
    fn test_xml_round_trip_16() {
        xml_round_trip_lexicon("16");
    }

    #[test]
    fn test_xml_round_trip_17() {
        xml_round_trip_lexicon("17");
    }

    #[test]
    fn test_xml_round_trip_18() {
        xml_round_trip_lexicon("18");
    }

    #[test]
    fn test_xml_round_trip_19() {
        xml_round_trip_entry("19");
    }

    #[test]
    fn test_xml_round_trip_20() {
        xml_round_trip_entry("20");
    }

    #[test]
    fn test_xml_round_trip_21() {
        xml_round_trip_entry("21");
    }

    #[test]
    fn test_xml_round_trip_22() {
        xml_round_trip_entry("22");
    }

    #[test]
    fn test_xml_round_trip_23() {
        xml_round_trip_entry("23");
    }

    #[test]
    fn test_xml_round_trip_24() {
        xml_round_trip_lexicon("24");
    }

    #[test]
    fn test_xml_round_trip_all_props() {
        xml_round_trip_lexicon("all_props");
    }
}
