//// Main Function
mod format;
mod identify;
mod process;
mod scheme;
pub mod utils;
pub mod data;
use crate::identify::{
    identify_vrtta,
};
use crate::data::{
    read_json_vrtta, VrttaData, read_json_matra, MatraData,
};
use crate::utils::{
    Input, UseParams, SearchParams, IdentifyParams, Params,
};
fn main() {
    //Input Verse in slp1 encoding
    let verse = "asty uttarasyAM diSi devatAtmA himAlayo nAma nagADirAjaH .
pUrvAparO toyaniDI vigAhya sTitaH pfTivyA iva mAnadaRqaH ..";

    //Print the input verse
    println!("\nInput Verse:\n{}\n\n", verse);

    // Make the Input Struct
    let input = Input::new(verse);
    println!("{:?}", input);

    let params = Params::new(IdentifyParams::IdentifyVrtta,
                             UseParams::UseExact,
                             SearchParams::SearchFuzzy);

    let vrtta_data = read_json_vrtta();

    let metre = identify_vrtta(input, vrtta_data, params);

}
