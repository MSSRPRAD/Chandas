//// Main Function
pub mod data;
mod format;
mod identify;
mod process;
mod scheme;
pub mod utils;
use crate::data::{read_json_matra, read_json_vrtta, MatraData, VrttaData};
use crate::identify::{identify_anushtup, identify_vrtta};
use crate::utils::{IdentifyParams, Input, Params};
fn main() {
    //Input Verse in slp1 encoding
    let verse = "tasya tvevaM praBAvasya
DarmajYasya mahAtmanaH .
sutArTaM tapyamAnasya
nAsIdvaMSakaraH sutaH .. 1 ..";

    //Print the input verse
    println!("\nInput Verse:\n{}\n\n", verse);

    // Make the Input Struct
    let input = Input::new(verse);
    println!("{:?}", input);

    let params = Params::new(IdentifyParams::IdentifyAnushtup);

    let vrtta_data = read_json_vrtta();

    let metre = identify_anushtup(input, vrtta_data, params);
    println!("\n\n{:?}\n\n", metre);
}
