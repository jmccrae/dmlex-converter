
mod serialization;
mod model;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };

    let serialized = serde_xml_rs::to_string(&point).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Point = serde_xml_rs::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}
