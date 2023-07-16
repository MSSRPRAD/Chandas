use crate::process::clean;
use crate::scheme::find_scheme;
use lazy_static::lazy_static;
type Sound = char;
//hrasva Vowels
const HRASVA: &str = "aiufx";
//dIrgha Vowels
const DIRGHA: &str = "AIUeEoOFX";
//Consonants
const HAL: &str = "kKgGNcCjJYwWqQRtTdDnpPbBmyrlvSzsh";
//Anusvara or Visarga
const OTHERS: &str = "MH";
//Sanskrit
const SANSKRIT: &str = "aAiIuUfFxXeEoOMHkKgGNcCjJYwWqQRtTdDnpPbBmyrlvSzshL";
#[derive(Debug)]
pub struct Input {
    pub PadaOne: Option<String>,
    pub PadaTwo: Option<String>,
    pub PadaThree: Option<String>,
    pub PadaFour: Option<String>,
    pub SchemeOne: Option<String>,
    pub SchemeTwo: Option<String>,
    pub SchemeThree: Option<String>,
    pub SchemeFour: Option<String>,
    pub SchemeOneTwo: Option<String>,
    pub SchemeThreeFour: Option<String>,
    pub SchemeAll: Option<String>,
}

impl Input {
    pub fn new(input_string: &str) -> Input {
        let lines: Vec<&str> = input_string.lines().collect();
        let mut input = Input {
            PadaOne: None,
            PadaTwo: None,
            PadaThree: None,
            PadaFour: None,
            SchemeOne: None,
            SchemeTwo: None,
            SchemeThree: None,
            SchemeFour: None,
            SchemeOneTwo: None,
            SchemeThreeFour: None,
            SchemeAll: None,
        };

        if let Some(line) = lines.get(0) {
            let processed = clean(line);
            let processed_clone = processed.to_owned();
            input.PadaOne = Some(processed_clone);
            input.SchemeOne = Some(find_scheme(&processed).to_string());
        }

        if let Some(line) = lines.get(1) {
            let processed = clean(line);
            let processed_clone = processed.to_owned();
            input.PadaTwo = Some(processed_clone);
            input.SchemeTwo = Some(find_scheme(&processed).to_string());

            input.SchemeOneTwo = input
                .PadaOne
                .as_ref()
                .and_then(|str1| input.PadaTwo.as_ref().map(|str2| str1.to_owned() + str2))
                .map(|s| find_scheme(&s).to_string());
        }

        if let Some(line) = lines.get(2) {
            let processed = clean(line);
            let processed_clone = processed.to_owned();
            input.PadaThree = Some(processed_clone);
            input.SchemeThree = Some(find_scheme(&processed).to_string());
        }
        if let Some(line) = lines.get(3) {
            let processed = clean(line);
            let processed_clone = processed.to_owned();
            input.PadaFour = Some(processed_clone);
            input.SchemeFour = Some(find_scheme(&processed).to_string());

            input.SchemeThreeFour = input
                .PadaThree
                .as_ref()
                .and_then(|str1| input.PadaFour.as_ref().map(|str2| str1.to_owned() + str2))
                .map(|s| find_scheme(&s).to_string());
        }
        input.SchemeAll = Some(find_scheme(&input_string).to_string());

        input
    }
    fn padas(&self) -> usize {
        let mut count = 0;

        if self.PadaOne.is_some() {
            count += 1;
        }

        if self.PadaTwo.is_some() {
            count += 1;
        }

        if self.PadaThree.is_some() {
            count += 1;
        }

        if self.PadaFour.is_some() {
            count += 1;
        }

        count
    }
}
#[derive(Debug)]
pub enum IdentifyParams {
    IdentifyAnushtup,
    IdentifyMatra,
    IdentifyVrtta,
    IdentifyAll,
}
#[derive(Debug)]
pub enum UseParams {
    UseExact,
    UseMerged,
}
#[derive(Debug)]
pub enum SearchParams {
    SearchFuzzy,
    SearchExact,
}

#[derive(Debug)]
pub struct Params {
    pub identify_params: IdentifyParams,
    pub use_params: UseParams,
    pub search_params: SearchParams,
}

impl Params {
    pub fn new(
        identify_params: IdentifyParams,
        use_params: UseParams,
        search_params: SearchParams,
    ) -> Params {
        Params {
            identify_params,
            use_params,
            search_params,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct IdentifyResult {
    pub name: String,
    pub description: String,
    pub similarity: usize,
    pub actual: Vec<String>,
    pub input: Vec<String>,
}
impl Ord for IdentifyResult {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.similarity.cmp(&self.similarity) // Reverse ordering for BinaryHeap
    }
}
impl PartialOrd for IdentifyResult {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
// A set of Sanskrit sounds.
//
// This implementation is copied directly from `vidyut_prakriya::sounds`. For details, see the
// comments there.
pub struct Set([u8; 256]);

impl Set {
    /// Creates an empty set.
    pub fn new() -> Self {
        Set([0; 256])
    }

    /// Creates a set whose members are the characters in `string`.
    pub fn from(string: impl AsRef<str>) -> Self {
        let mut res = Self::new();
        for c in string.as_ref().chars() {
            res.0[c as usize] = 1;
        }
        res
    }

    // Returns whether the set contains the given sound.
    pub fn contains(&self, c: Sound) -> bool {
        self.0[c as usize] == 1
    }
}

// Helper functions to identify the type of the sanskrit sound....

pub fn is_hrasva(c: Sound) -> bool {
    lazy_static! {
        static ref CHARS: Set = Set::from(HRASVA);
    }
    CHARS.contains(c)
}

pub fn is_dirgha(c: Sound) -> bool {
    lazy_static! {
        static ref CHARS: Set = Set::from(DIRGHA);
    }
    CHARS.contains(c)
}

pub fn is_hal(c: Sound) -> bool {
    lazy_static! {
        static ref CHARS: Set = Set::from(HAL);
    }
    CHARS.contains(c)
}

pub fn is_special(c: Sound) -> bool {
    lazy_static! {
        static ref CHARS: Set = Set::from(OTHERS);
    }
    CHARS.contains(c)
}

/// Copied from sounds.rs in vidyut-sandhi crate
/// Returns whether the given character is a Sanskrit sound or not.
/// Non-Sanskrit sounds include:
/// s - avagraha
/// - spaces
/// - other punctuation characters (|, ||, numbers)
/// - characters or symbols from non-SLP1 encodings
pub fn is_sanskrit(c: char) -> bool {
    lazy_static! {
        static ref CHARS: Set = Set::from(SANSKRIT);
    }
    CHARS.contains(c)
}
