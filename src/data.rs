use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

#[derive(Deserialize, Debug, Serialize)]
pub struct VrttaData {
    pub comment: Vec<String>,
    pub metres: Vec<Vrtta>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Vrtta {
    pub name: String,
    pub pattern: StringOrList,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct MatraData {
    pub comment: Vec<String>,
    pub metres: Vec<Matra>,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Matra {
    pub name: String,
    pub pattern: MData,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct MData {
    pub regex: Vec<String>,
    pub comment: String,
}

#[derive(Deserialize, Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum StringOrList {
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
