use serde::Deserialize;
use serde_json;
use std::fs;

#[derive(Deserialize)]
pub struct VrttaData {
    comment: Vec<String>,
    metres: Vec<Vrtta>,
}

#[derive(Clone, Deserialize)]
struct Vrtta {
    name: String,
    pattern: StringOrList,
}

#[derive(Clone, Deserialize)]
pub struct MatraData {
    comment: Vec<String>,
    metres: Vec<Matra>,
}

#[derive(Clone, Deserialize, Debug)]
struct Matra {
    name: String,
    pattern: MData,
}

#[derive(Clone, Deserialize, Debug)]
struct MData {
    regex: Vec<String>,
    comment: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
enum StringOrList {
    String(String),
    List(Vec<String>),
}

//// Function that reads the json data, stores it in an object and
/// returns that object

pub fn read_json_vrtta() -> VrttaData {
    let path = "./src/data/mishra.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let my_data: VrttaData = serde_json::from_str(&data).expect("Unable to parse");

    // I am throwing away the value
    let _ = my_data.comment;

    return my_data;
}

pub fn read_json_matra() -> MatraData {
    let path = "./src/data/matra.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let my_data: MatraData = serde_json::from_str(&data).expect("Unable to parse");
    return my_data;
}
