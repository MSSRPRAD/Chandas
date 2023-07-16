//// Main Function
mod format;
mod identify;
mod process;
mod scheme;
pub mod utils;
pub mod data;
use crate::identify::{
    identify,
};
use crate::data::{
    read_json_vrtta, VrttaData, read_json_matra, MatraData,
};
use crate::utils::{
    Input, SearchParams::{MergedSearch, ExactSearch, ExtraSearch}, IdentifyParams, Params,
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

    let params = Params::new(
        IdentifyParams::IdentifyVrtta,
        vec![MergedSearch, ExactSearch, ExtraSearch]
    );

    let vrtta_data = read_json_vrtta();

    let metre = identify(input, vrtta_data, params);
    // println!("\n\n{:?}\n\n", metre);
}
