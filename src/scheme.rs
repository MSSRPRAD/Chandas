//// Goal is to take an slp1 encoded sanskrit verse and extract the
//// G-L pattern from it.
use crate::utils::{is_dirgha, is_hal, is_hrasva, is_special};

// Returning the Guru Laghu Scheme of the String
pub fn find_scheme(raw: &str) -> &'static str {
    let mut scheme = String::new();
    for i in 0..raw.len() {
        let curr: char = raw.chars().nth(i).unwrap();
        //Other than from second last char
        if i <= raw.len() - 3 {
            let next: char = raw.chars().nth(i + 1).unwrap();
            let next_next: char = raw.chars().nth(i + 2).unwrap();
            if is_dirgha(curr) {
                scheme.push('G');
            } else if is_hrasva(curr) && is_hal(next) && is_hal(next_next) {
                scheme.push('G');
            } else if is_hrasva(curr) && is_special(next) {
                scheme.push('G');
            } else if is_hrasva(curr) {
                scheme.push('L');
            }
        } else if i == raw.len() - 2 {
            let next: char = raw.chars().nth(i + 1).unwrap();
            //From second last character to last character it is Laghu only if
            //Dirgha vowel and followed by anusvara/visarga.
            if is_dirgha(curr) {
                scheme.push('G');
            } else if is_hrasva(curr) && is_special(next) {
                scheme.push('G');
            } else if is_hrasva(curr) {
                scheme.push('L');
            }
        } else {
            if is_dirgha(curr) {
                scheme.push('G');
            } else if is_hrasva(curr) {
                scheme.push('L');
            }
        }
    }
    let boxed_str: Box<str> = scheme.into_boxed_str();
    Box::leak(boxed_str)
}
