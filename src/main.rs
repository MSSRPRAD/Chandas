//// Main Function
mod format;
mod identify;
mod process;
mod scheme;
pub mod utils;

use crate::utils::Input;
fn main() {
    //Input Verse in slp1 encoding
    let verse = "asty uttarasyAM diSi devatAtmA himAlayo nAma nagADirAjaH .
pUrvAparO toyaniDI vigAhya sTitaH pfTivyA iva mAnadaRqaH ..";

    //Print the input verse
    println!("\nInput Verse:\n{}\n\n", verse);

    // Make the Input Struct
    let input = Input::new(verse);
    println!("{:?}", input);
}
