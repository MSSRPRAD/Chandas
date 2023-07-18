//// Main Function
pub mod data;
mod format;
mod identify;
mod process;
mod scheme;
pub mod utils;
use crate::data::{read_json_matra, read_json_vrtta, MatraData, VrttaData};
use crate::identify::{identify_anushtup, identify_matra, identify_vrtta};
use crate::utils::{IdentifyParams, Input, Params};
use pyo3::pymodule;
use serde::Serialize;
use serde_json::to_string;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

#[derive(Debug)]
struct InputVerse {
    verse: String,
}

impl InputVerse {
    fn new(verse: String) -> Self {
        InputVerse { verse }
    }
}


#[pyfunction]
fn identify_anushtup_rs(verse: String) -> PyResult<String> {
    //Input Verse in slp1 encoding
    let verse = &verse;

    //Print the input verse
    println!("\nInput Verse:\n{}\n\n", verse);

    // Make the Input Struct
    let input = Input::new(verse);
    println!("{:?}", input);

    let matra_data = read_json_vrtta();

    let metre = identify_anushtup(input, matra_data);
    println!("\n\n{:?}\n", metre);

    let res = to_string(&metre).unwrap();
    return Ok(res);
}


#[pymodule]
fn chandas(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(identify_anushtup_rs, m)?)?;
    Ok(())
}
