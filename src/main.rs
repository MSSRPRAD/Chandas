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
fn main() {
    //Input Verse in slp1 encoding
    let verse = "asty uttArasyAM dISi devatAtmA
himAlayo nAma nagADirAjaH .
pUrvAparO toyaniDI vigAhya
sTitaH pfTivyA iva mAnadaRqaH ..";

    //Print the input verse
    println!("\nInput Verse:\n{}\n\n", verse);

    // Make the Input Struct
    let input = Input::new(verse);
    println!("{:?}", input);

    let matra_data = read_json_vrtta();

    let metre = identify_vrtta(input, matra_data);
    println!("\n\n{:?}\n", metre);
}
