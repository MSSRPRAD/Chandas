// Goal is to take a string of SLP1 transliterated Sanskrit text
// and clean it (Remove Spaces, Remove tags, Remove Special Characters)
use crate::utils::is_sanskrit;

// Remove all non-sanskrit sounds from the input strings.
pub fn clean(raw: &str) -> String {
    let mut cleaned = String::new();

    for i in raw.chars() {
        if is_sanskrit(i) {
            cleaned.push(i);
        }
    }

    cleaned
}
