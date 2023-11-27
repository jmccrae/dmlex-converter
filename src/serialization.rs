/// Any serialization specific code should be placed in this module.

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
