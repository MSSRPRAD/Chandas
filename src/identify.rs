//// This code has two components.
/// 1) Read the JSON data from https://github.com/shreevatsa/sanskrit/blob/master/data/mishra.json (saved locally)
///    and store it as a rust object
/// 2) Take the scheme of the input verse as the input and
///     check against the data for the metre
extern crate levenshtein;
use crate::data::{StringOrList, Vrtta, VrttaData};
use crate::utils::{IdentifyParams, Input, MatchTracker, MatchType, Params, PatternMatch};
use levenshtein::levenshtein;
// use crate::scheme::Me
use crate::data::Matra;
use crate::MatraData;

fn scheme_exists(scheme: &Option<String>) -> bool {
    scheme.is_some()
}

pub fn check_individual_match(input: Input, metre: Vrtta) -> Vec<PatternMatch> {
    let mut actual = get_actual(metre.clone().pattern);
    let mut matches: Vec<PatternMatch> = Vec::new();
    let match_types = vec![
        MatchType::IndividualPadaOne,
        MatchType::IndividualPadaTwo,
        MatchType::IndividualPadaThree,
        MatchType::IndividualPadaFour,
    ];

    // Individual Pada 1
    if scheme_exists(&input.SchemeOne) {
        let len1 = actual[0].len();
        // Gante Condition for Pada1
        if input.SchemeOne.clone().unwrap().len() == actual[0].len() {
            actual[0].replace_range(
                len1 - 1..len1,
                &input
                    .SchemeOne
                    .clone()
                    .unwrap()
                    .chars()
                    .last()
                    .unwrap()
                    .to_string(),
            );
        }
        matches.push(PatternMatch {
            metre: metre.clone(),
            match_type: match_types[0],
            quality: 1.0
                - levenshtein(&input.SchemeOne.clone().unwrap(), &actual[0]) as f64
                    / input.SchemeOne.clone().unwrap().len() as f64,
            pada: Some(1),
        });
        matches.push(PatternMatch {
            metre: metre.clone(),
            match_type: match_types[0],
            quality: 1.0
                - levenshtein(&input.SchemeOne.clone().unwrap(), &actual[1]) as f64
                    / input.SchemeOne.clone().unwrap().len() as f64,
            pada: Some(2),
        });
        matches.push(PatternMatch {
            metre: metre.clone(),
            match_type: match_types[0],
            quality: 1.0
                - levenshtein(&input.SchemeOne.clone().unwrap(), &actual[2]) as f64
                    / input.SchemeOne.clone().unwrap().len() as f64,
            pada: Some(3),
        });
        matches.push(PatternMatch {
            metre: metre.clone(),
            match_type: match_types[0],
            quality: 1.0
                - levenshtein(&input.SchemeOne.clone().unwrap(), &actual[3]) as f64
                    / input.SchemeOne.clone().unwrap().len() as f64,
            pada: Some(4),
        });
    }
    // Second Pada
    if scheme_exists(&input.SchemeTwo) {
        let len2 = actual[1].len();
        // Gante Condition for Pada2
        if input.SchemeTwo.clone().unwrap().len() == actual[1].len() {
            actual[1].replace_range(
                len2 - 1..len2,
                &input
                    .SchemeTwo
                    .clone()
                    .unwrap()
                    .chars()
                    .last()
                    .unwrap()
                    .to_string(),
            );
        }
        matches.push(PatternMatch {
            metre: metre.clone(),
            match_type: match_types[1],
            quality: 1.0
                - levenshtein(&input.SchemeTwo.clone().unwrap(), &actual[0]) as f64
                    / input.SchemeTwo.clone().unwrap().len() as f64,
            pada: Some(1),
        });
        matches.push(PatternMatch {
            metre: metre.clone(),
            match_type: match_types[1],
            quality: 1.0
                - levenshtein(&input.SchemeTwo.clone().unwrap(), &actual[1]) as f64
                    / input.SchemeTwo.clone().unwrap().len() as f64,
            pada: Some(2),
        });
        matches.push(PatternMatch {
            metre: metre.clone(),
            match_type: match_types[1],
            quality: 1.0
                - levenshtein(&input.SchemeTwo.clone().unwrap(), &actual[2]) as f64
                    / input.SchemeTwo.clone().unwrap().len() as f64,
            pada: Some(3),
        });
        matches.push(PatternMatch {
            metre: metre.clone(),
            match_type: match_types[1],
            quality: 1.0
                - levenshtein(&input.SchemeTwo.clone().unwrap(), &actual[3]) as f64
                    / input.SchemeTwo.clone().unwrap().len() as f64,
            pada: Some(4),
        });
    }
    // Pada 3
    if scheme_exists(&input.SchemeThree) {
        let len3 = actual[2].len();
        // Gante Condition for Pada3
        if input.SchemeThree.clone().unwrap().len() == actual[2].len() {
            actual[2].replace_range(
                len3 - 1..len3,
                &input
                    .SchemeThree
                    .clone()
                    .unwrap()
                    .chars()
                    .last()
                    .unwrap()
                    .to_string(),
            );
        }
        matches.push(PatternMatch {
            metre: metre.clone(),
            match_type: match_types[2],
            quality: 1.0
                - levenshtein(&input.SchemeThree.clone().unwrap(), &actual[0]) as f64
                    / input.SchemeThree.clone().unwrap().len() as f64,
            pada: Some(1),
        });
        matches.push(PatternMatch {
            metre: metre.clone(),
            match_type: match_types[2],
            quality: 1.0
                - levenshtein(&input.SchemeThree.clone().unwrap(), &actual[1]) as f64
                    / input.SchemeThree.clone().unwrap().len() as f64,
            pada: Some(2),
        });
        matches.push(PatternMatch {
            metre: metre.clone(),
            match_type: match_types[2],
            quality: 1.0
                - levenshtein(&input.SchemeThree.clone().unwrap(), &actual[2]) as f64
                    / input.SchemeThree.clone().unwrap().len() as f64,
            pada: Some(3),
        });
        matches.push(PatternMatch {
            metre: metre.clone(),
            match_type: match_types[2],
            quality: 1.0
                - levenshtein(&input.SchemeThree.clone().unwrap(), &actual[3]) as f64
                    / input.SchemeThree.clone().unwrap().len() as f64,
            pada: Some(4),
        });
    }
    // Pada 4
    if scheme_exists(&input.SchemeFour) {
        let len4 = actual[3].len();
        // Gante Condition for Pada4
        if input.SchemeFour.clone().unwrap().len() == actual[3].len() {
            actual[3].replace_range(
                len4 - 1..len4,
                &input
                    .SchemeFour
                    .clone()
                    .unwrap()
                    .chars()
                    .last()
                    .unwrap()
                    .to_string(),
            );
        }
        matches.push(PatternMatch {
            metre: metre.clone(),
            match_type: match_types[3],
            quality: 1.0
                - levenshtein(&input.SchemeFour.clone().unwrap(), &actual[0]) as f64
                    / input.SchemeFour.clone().unwrap().len() as f64,
            pada: Some(1),
        });
        matches.push(PatternMatch {
            metre: metre.clone(),
            match_type: match_types[3],
            quality: 1.0
                - levenshtein(&input.SchemeFour.clone().unwrap(), &actual[1]) as f64
                    / input.SchemeFour.clone().unwrap().len() as f64,
            pada: Some(2),
        });
        matches.push(PatternMatch {
            metre: metre.clone(),
            match_type: match_types[3],
            quality: 1.0
                - levenshtein(&input.SchemeFour.clone().unwrap(), &actual[2]) as f64
                    / input.SchemeFour.clone().unwrap().len() as f64,
            pada: Some(3),
        });
        matches.push(PatternMatch {
            metre: metre.clone(),
            match_type: match_types[3],
            quality: 1.0
                - levenshtein(&input.SchemeFour.clone().unwrap(), &actual[3]) as f64
                    / input.SchemeFour.clone().unwrap().len() as f64,
            pada: Some(4),
        });
    }
    matches
}

pub fn replace_x(first_string: String, second_string: String) -> String {
    let mut first_string = first_string.clone();
    for (index, char_first) in first_string.clone().chars().enumerate() {
        if char_first == 'X' {
            if let Some(char_second) = second_string.chars().nth(index) {
                first_string.replace_range(index..index + 1, &char_second.to_string());
            }
        }
    }
    first_string
}

pub fn check_whole_match_anushtup(input: Input, metre: Vrtta) -> PatternMatch {
    let mut actual = get_actual(metre.clone().pattern).concat();
    let actual1 = get_actual(metre.clone().pattern);
    let l1 = actual1[0].len();
    let l2 = actual1[1].len();
    let l3 = actual1[2].len();
    let l4 = actual1[3].len();
    // Seperate condition to check when the difference could be that of gante (when the dimensions of both are same)
    if &actual.len() == &input.SchemeAll.clone().unwrap().len() {
        // let actual1 = actual.clone();
        actual.replace_range(
            l1 - 1..l1,
            &input
                .SchemeAll
                .clone()
                .unwrap()
                .chars()
                .nth(l1 - 1)
                .unwrap()
                .to_string(),
        );
        actual.replace_range(
            l1 + l2 - 1..l1+l2,
            &input
                .SchemeAll
                .clone()
                .unwrap()
                .chars()
                .nth(l1 + l2 - 1)
                .unwrap()
                .to_string(),
        );
        actual.replace_range(
            l1 + l2 + l3 - 1..l1+l2+l3,
            &input
                .SchemeAll
                .clone()
                .unwrap()
                .chars()
                .nth(l1 + l2 + l3 - 1)
                .unwrap()
                .to_string(),
        );
        actual.replace_range(
            l1 + l2 + l3 + l4 - 1..l1+l2+l3+l4,
            &input
                .SchemeAll
                .clone()
                .unwrap()
                .chars()
                .nth(l1 + l2 + l3 + l4 - 1)
                .unwrap()
                .to_string(),
        );
    }
    println!("\nreached here!!!\n");
    let found = replace_x(actual.clone(), input.SchemeAll.clone().unwrap());
    println!(
        "Checking b/w: \n{:?}\nand\n{:?}",
        &input.SchemeAll.clone().unwrap(),
        found
    );
    let distance = levenshtein(&input.SchemeAll.clone().unwrap(), &found) as f64;
    let quality = 1.0 - distance / (actual.len() as f64);
    PatternMatch {
        metre: metre,
        match_type: MatchType::WholePada,
        quality: quality,
        pada: None,
    }
}

pub fn check_whole_match(input: Input, metre: Vrtta) -> PatternMatch {
    let mut actual = get_actual(metre.clone().pattern).concat();
    let actual1 = get_actual(metre.clone().pattern);
    let l1 = actual1[0].len();
    let l2 = actual1[1].len();
    let l3 = actual1[2].len();
    let l4 = actual1[3].len();
    // Seperate condition to check when the difference could be that of gante (when the dimensions of both are same)
    if &actual.len() == &input.SchemeAll.clone().unwrap().len() {
        // let actual1 = actual.clone();
        actual.replace_range(
            l1 - 1..l1,
            &input
                .SchemeAll
                .clone()
                .unwrap()
                .chars()
                .nth(l1 - 1)
                .unwrap()
                .to_string(),
        );
        actual.replace_range(
            l1 + l2 - 1..l1+l2,
            &input
                .SchemeAll
                .clone()
                .unwrap()
                .chars()
                .nth(l1 + l2 - 1)
                .unwrap()
                .to_string(),
        );
        actual.replace_range(
            l1 + l2 + l3 - 1..l1+l2+l3,
            &input
                .SchemeAll
                .clone()
                .unwrap()
                .chars()
                .nth(l1 + l2 + l3 - 1)
                .unwrap()
                .to_string(),
        );
        actual.replace_range(
            l1 + l2 + l3 + l4 - 1..l1+l2+l3+l4,
            &input
                .SchemeAll
                .clone()
                .unwrap()
                .chars()
                .nth(l1 + l2 + l3 + l4 - 1)
                .unwrap()
                .to_string(),
        );
    }
    let distance = levenshtein(&actual, &input.SchemeAll.unwrap()) as f64;
    let quality = 1.0 - distance / (actual.len() as f64);
    PatternMatch {
        metre: metre,
        match_type: MatchType::WholePada,
        quality: quality,
        pada: None,
    }
}

pub fn check_pair_match(input: Input, metre: Vrtta) -> Vec<PatternMatch> {
    let actual = get_actual(metre.clone().pattern);
    let mut matches: Vec<PatternMatch> = Vec::new();
    let match_types = vec![MatchType::PadaOneTwo, MatchType::PadaThreeFour];
    if scheme_exists(&input.SchemeOneTwo) {
        matches.push(PatternMatch {
            metre: metre.clone(),
            match_type: match_types[0],
            quality: 1.0
                - levenshtein(&input.SchemeOneTwo.clone().unwrap(), &actual[0]) as f64
                    / input.SchemeOneTwo.unwrap().len() as f64,
            pada: Some(12),
        })
    }
    if scheme_exists(&input.SchemeThreeFour) {
        matches.push(PatternMatch {
            metre: metre.clone(),
            match_type: match_types[1],
            quality: 1.0
                - levenshtein(&input.SchemeThreeFour.clone().unwrap(), &actual[1]) as f64
                    / input.SchemeThreeFour.unwrap().len() as f64,
            pada: Some(34),
        })
    }
    matches
}

fn get_actual(string_or_list: StringOrList) -> Vec<String> {
    match string_or_list {
        StringOrList::String(s) => vec![s; 4], // Repeat the string four times
        StringOrList::List(list) => {
            match list.len() {
                1 => vec![list[0].clone(); 4], // Repeat the single string four times
                2 => {
                    let mut actual = Vec::new();
                    for _ in 0..2 {
                        actual.push(list[0].clone());
                        actual.push(list[1].clone());
                    }
                    actual // Return the 4-row array from the two strings used twice
                }
                4 => list, // Return the original list of 4 strings
                _ => panic!("Invalid input: List must contain either 1, 2, or 4 strings"),
            }
        }
    }
}

pub fn identify_anushtup(input: Input, vrtta_data: VrttaData) -> MatchTracker {
    let mut tracker = MatchTracker::new(1);
    let anushtubh = &vrtta_data.metres[0];
    let metre = anushtubh.clone();
    // let ind_pada_res = check_individual_match_anushtup(input.clone(), metre.clone());
    // let match_types = vec![
    //     MatchType::IndividualPadaOne,
    //     MatchType::IndividualPadaTwo,
    //     MatchType::IndividualPadaThree,
    //     MatchType::IndividualPadaFour,
    // ];
    // for (match_type, pattern_match) in match_types.into_iter().zip(ind_pada_res.into_iter()) {
    //     tracker.add_match(match_type, pattern_match);
    // }

    // Add the WholePattern Match
    tracker.add_match(
        MatchType::WholePada,
        check_whole_match_anushtup(input.clone(), metre.clone()),
    );

    println!("{:?}", anushtubh);
    tracker
}

pub fn identify_vrtta(input: Input, vrtta_data: VrttaData) -> MatchTracker {
    println!(
        "No of vrtta metres in the database: \n{}\n\n",
        vrtta_data.metres.len()
    );
    // Results
    let mut tracker = MatchTracker::new(3);
    // matches = PatternMatch;

    // Skip Anushtubh and go through all the other metres
    for metre in vrtta_data.metres.iter().skip(1) {
        // Find the qualities
        // Add the IndividualPada Match
        let ind_pada_res = check_individual_match(input.clone(), metre.clone());
        let match_types = vec![
            MatchType::IndividualPadaOne,
            MatchType::IndividualPadaTwo,
            MatchType::IndividualPadaThree,
            MatchType::IndividualPadaFour,
        ];
        for (match_type, pattern_match) in match_types.into_iter().zip(ind_pada_res.into_iter()) {
            tracker.add_match(match_type, pattern_match);
        }
        // Add the WholePattern Match
        tracker.add_match(
            MatchType::WholePada,
            check_whole_match(input.clone(), metre.clone()),
        );
        // println!("Metre Name: {}", metre.name);
        // println!("Pattern: {:?}", metre.pattern);
    }
    return tracker;
}

pub fn identify_matra(input: Input, matra_data: MatraData) -> Option<Matra> {
    let s: Vec<char> = input.SchemeAll.unwrap().chars().collect();
    // Find Input scheme's cumulative frequency
    let mut freq_input = Vec::new();

    for i in 0..s.len() {
        if s[i] == 'L' {
            freq_input.push(1);
        } else if s[i] == 'G' {
            freq_input.push(2);
        }

        if i > 0 {
            freq_input[i] += freq_input[i - 1];
        }
    }

    for i in 0..matra_data.metres.len() {
        let name = &matra_data.metres[i].name;
        // Check if the freq_input satisfies the scheme

        // First we generate all the 4 possible types for each jati based
        // on whether the 2nd and 4th pada exception occurs or not. 2x2 = 4.
        let mut type1 = Vec::<usize>::new();
        let mut type2 = Vec::<usize>::new();
        let mut type3 = Vec::<usize>::new();
        let mut type4 = Vec::<usize>::new();

        for j in 0..4 {
            for k in 0..matra_data.metres[i].pattern.regex[j].len() {
                match matra_data.metres[i].pattern.regex[j]
                    .chars()
                    .nth(k)
                    .unwrap()
                {
                    '4' => {
                        type1.push(4);
                        type2.push(4);
                        type3.push(4);
                        type4.push(4);
                    }
                    '2' => {
                        type1.push(2);
                        type2.push(2);
                        type3.push(2);
                        type4.push(2);
                    }
                    'L' => {
                        type1.push(1);
                        type2.push(1);
                        type3.push(1);
                        type4.push(1);
                    }
                    '1' => {
                        type1.push(1);
                        type2.push(1);
                        type3.push(1);
                        type4.push(1);
                    }
                    '.' => {
                        if j == 1 {
                            type1.push(2);
                            type2.push(2);
                            type3.push(1);
                            type4.push(1);
                        } else if j == 3 {
                            type1.push(2);
                            type2.push(1);
                            type3.push(2);
                            type4.push(1);
                        }
                    }
                    _ => {}
                }
            }
        }
        // Convert all to cumulative frequencies
        for k in 1..type1.len() {
            type1[k] += type1[k - 1];
            type2[k] += type2[k - 1];
            type3[k] += type3[k - 1];
            type4[k] += type4[k - 1];
        }
        /* if freq_input[freq_input.len()-1] != type1[type1.len()-1] ||freq_input[freq_input.len()-1] != type2[type2.len()-1] ||freq_input[freq_input.len()-1] != type3[type3.len()-1] ||freq_input[freq_input.len()-1] != type4[type4.len()-1] {
            return String::from("Metre Not Found! Sorry for that :(");
        }
        */
        if freq_contains(&type1, &freq_input) && freq_input.last().unwrap() == type1.last().unwrap()
            || freq_contains(&type2, &freq_input)
                && freq_input.last().unwrap() == type2.last().unwrap()
            || freq_contains(&type3, &freq_input)
                && freq_input.last().unwrap() == type3.last().unwrap()
            || freq_contains(&type4, &freq_input)
                && freq_input.last().unwrap() == type4.last().unwrap()
        {
            return Some(matra_data.metres[i].clone());
        }
    }

    None
}

fn freq_contains(a: &Vec<usize>, b: &Vec<usize>) -> bool {
    for i in 0..a.len() {
        if !b.contains(&a[i]) {
            return false;
        }
    }
    true
}
