//// This code has two components.
/// 1) Read the JSON data from https://github.com/shreevatsa/sanskrit/blob/master/data/mishra.json (saved locally)
///    and store it as a rust object
/// 2) Take the scheme of the input verse as the input and
///     check against the data for the metre
extern crate levenshtein;
// use crate::scheme::Metre;

//// Structs that store the metrical data from mishra.json

// Function that matches the scheme against some pattern from the data
// For now it only matches the first pada of the scheme with
// the first pattern string. So in effect it works only when all 4 padas
// have same pattern and गन्ते is ignored.

// pub fn similarity_to_vrtta(a: &Vec<String>, s: &Vec<Metre>) -> usize {

//     println!("SOME ERROR OCCURED! :(");
//     return 99999;
// }

// pub fn is_sama_vrtta(s: &Vec<Metre>) -> bool {
//     let pada_len = s.len() / 4;
//     for i in 1..4 {
//         for j in 0..pada_len - 1 {
//             if s[j].unwrap() != s[i * pada_len + j].unwrap() {
//                 return false;
//             }
//         }
//     }
//     return true;
// }

//// Function that takes the scheme as the input and returns
//// the name of the metre
//// To do this it reads the JSON file through read_json()

// pub fn identify(s: &Vec<Metre>) -> String {
//     if is_sama_vrtta(s) {
//         println!("Input is a sama vRtta Metre!");
//     }

//     let vrtta_kosha: VrttaData = read_json();
//     let mut min_distance = 99999;
//     let mut closest_metre_name = String::new();
//     let mut scheme_of_closest_pattern_ardha = Vec::new();
//     let mut scheme_of_closest_pattern_sama = Vec::new();
//     let mut closest_is_sama_vrtta = true;
//     for i in 0..vrtta_kosha.metres.len() {
//         let ref metre_name = vrtta_kosha.metres[i].name;
//         //// Find each pattern as a vector of strings.
//         //// Right now it is being stored as either
//         //// 1) String or 2) List of Strings

//         let mut vec = Vec::new();

//         match &vrtta_kosha.metres[i].pattern {
//             StringOrList::String(a) => {
//                 vec.push(String::from(a));
//             }
//             StringOrList::List(b) => {
//                 vec = b.to_vec();
//             }
//         }

//         let padas = vec.len();
//         //// Check even if it is not a sama vrtta because user can make mistake while writing input
//         if padas == 1 {
//             let tmp = identify_sama_vrtta(&vec[0], s);
//             if tmp < min_distance {
//                 min_distance = tmp;
//                 closest_metre_name = String::from(metre_name.to_string());
//                 scheme_of_closest_pattern_sama.push(vec.clone());
//                 println!("The scheme is: ");
//                 println!("{:?}", scheme_of_closest_pattern_sama);
//                 closest_is_sama_vrtta = true;
//             }
//             if min_distance == 0 {
//                 println!("The scheme is: ");
//                 println!("{:?}", scheme_of_closest_pattern_sama);
//                 return String::from(metre_name.to_string());
//             }
//         }
//         // Check for ardha sama vrtta
//         if padas > 1 {
//             let tmp = identify_ardha_sama_vrtta(&vec, s);
//             if tmp < min_distance {
//                 min_distance = tmp;
//                 closest_metre_name = String::from(metre_name.to_string());
//                 scheme_of_closest_pattern_ardha.push(vec);
//                 closest_is_sama_vrtta = false;
//             }
//             if min_distance == 0 {
//                 if closest_metre_name != "anushtubh" {
//                     println!("Input is an ardha-sama-vrtta.");
//                 }
//                 println!("The scheme is: ");
//                 println!("{:?}", scheme_of_closest_pattern_ardha);
//                 return String::from(closest_metre_name);
//             }
//         }
//     }

//     //// If no Pattern found
//     println!("A similar metre is: {:?}", closest_metre_name);
//     println!(
//         "It's levenschtein distance from the input is: {:?}",
//         min_distance
//     );
//     println!("The scheme is: ");
//     if closest_is_sama_vrtta {
//         println!("{:?}", scheme_of_closest_pattern_sama);
//     } else {
//         println!("{:?}", scheme_of_closest_pattern_ardha);
//     }

//     String::from("Metre Not Found! Sorry for that :(")
// }

// pub fn identify_matra(s: &Vec<Metre>) -> String {
//     // Read JSON
//     let matra_kosha: MatraData = read_json_matra();

//     // Find Input scheme's cumulative frequency
//     let mut freq_input = Vec::new();

//     for i in 0..s.len() {
//         if s[i].unwrap() == 'L' {
//             freq_input.push(1);
//         } else if s[i].unwrap() == 'G' {
//             freq_input.push(2);
//         }

//         if i > 0 {
//             freq_input[i] += freq_input[i - 1];
//         }
//     }

//     for i in 0..matra_kosha.metres.len() {
//         let name = &matra_kosha.metres[i].name;
//         // Check if the freq_input satisfies the scheme

//         // First we generate all the 4 possible types for each jati based
//         // on whether the 2nd and 4th pada exception occurs or not. 2x2 = 4.
//         let mut type1 = Vec::<usize>::new();
//         let mut type2 = Vec::<usize>::new();
//         let mut type3 = Vec::<usize>::new();
//         let mut type4 = Vec::<usize>::new();

//         for j in 0..4 {
//             for k in 0..matra_kosha.metres[i].pattern.regex[j].len() {
//                 match matra_kosha.metres[i].pattern.regex[j]
//                     .chars()
//                     .nth(k)
//                     .unwrap()
//                 {
//                     '4' => {
//                         type1.push(4);
//                         type2.push(4);
//                         type3.push(4);
//                         type4.push(4);
//                     }
//                     '2' => {
//                         type1.push(2);
//                         type2.push(2);
//                         type3.push(2);
//                         type4.push(2);
//                     }
//                     'L' => {
//                         type1.push(1);
//                         type2.push(1);
//                         type3.push(1);
//                         type4.push(1);
//                     }
//                     '1' => {
//                         type1.push(1);
//                         type2.push(1);
//                         type3.push(1);
//                         type4.push(1);
//                     }
//                     '.' => {
//                         if j == 1 {
//                             type1.push(2);
//                             type2.push(2);
//                             type3.push(1);
//                             type4.push(1);
//                         } else if j == 3 {
//                             type1.push(2);
//                             type2.push(1);
//                             type3.push(2);
//                             type4.push(1);
//                         }
//                     }
//                     _ => {}
//                 }
//             }
//         }
//         // Convert all to cumulative frequencies
//         for k in 1..type1.len() {
//             type1[k] += type1[k - 1];
//             type2[k] += type2[k - 1];
//             type3[k] += type3[k - 1];
//             type4[k] += type4[k - 1];
//         }
//         /* if freq_input[freq_input.len()-1] != type1[type1.len()-1] ||freq_input[freq_input.len()-1] != type2[type2.len()-1] ||freq_input[freq_input.len()-1] != type3[type3.len()-1] ||freq_input[freq_input.len()-1] != type4[type4.len()-1] {
//             return String::from("Metre Not Found! Sorry for that :(");
//         }
//         */
//         if freq_contains(&type1, &freq_input) && freq_input.last().unwrap() == type1.last().unwrap()
//             || freq_contains(&type2, &freq_input)
//                 && freq_input.last().unwrap() == type2.last().unwrap()
//             || freq_contains(&type3, &freq_input)
//                 && freq_input.last().unwrap() == type3.last().unwrap()
//             || freq_contains(&type4, &freq_input)
//                 && freq_input.last().unwrap() == type4.last().unwrap()
//         {
//             return String::from(name);
//         }
//     }

//     String::from("Metre Not Found! Sorry for that :(")
// }

// fn freq_contains(a: &Vec<usize>, b: &Vec<usize>) -> bool {
//     for i in 0..a.len() {
//         if !b.contains(&a[i]) {
//             return false;
//         }
//     }
//     true
// }
