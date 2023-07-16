// Goal is to take a slp1 encoded verse, it's identified metre, and add
// '\n' at pada ends and also to display the scheme of the metre in G-L-Y format (From the database)

//Format the verse output such that each syllable starts with consonants and ends with a vowel. Also accounts for H and M
// pub fn format_verse(verse: &String, s: &Vec<scheme::Metre>) -> String {
//     let mut res: String = String::new();
//     let mut i: usize = 0;
//     let mut count: usize = 0;
//     while i < verse.len() - 1 {
//         let c: char = verse.chars().nth(i).unwrap();
//         // print!("{}", c);
//         res.push(c);
//         if is_dirgha(c) || is_hrasva(c) {
//             if !is_special(verse.chars().nth(i + 1).unwrap()) {
//                 // print!(" ");
//                 res.push_str(&" ");
//                 count += 1;
//                 if count == s.len() / 2 || count == s.len() / 4 || count == 3 * s.len() / 4 {
//                     // println!();
//                     res.push('\n');
//                 }
//             } else {
//                 // print!("{} ", verse.chars().nth(i+1).unwrap());
//                 res.push(verse.chars().nth(i + 1).unwrap());
//                 res.push(' ');
//                 count += 1;
//                 if count == s.len() / 2 || count == s.len() / 4 || count == 3 * s.len() / 4 {
//                     // println!();
//                     res.push('\n');
//                 }
//                 i += 1;
//             }
//         }
//         i += 1;
//     }
//     res.push(verse.chars().nth(verse.len() - 1).unwrap());
//     res.push('\n');

//     res
// }

// // Print the scheme of the verse
// pub fn print_scheme(s: &Vec<scheme::Metre>) {
//     for i in 0..s.len() {
//         print!("{:?}", s[i]);
//         if i == s.len() / 2 - 1 || i == s.len() / 4 - 1 || i == 3 * s.len() / 4 - 1 {
//             println!();
//         } else if i != s.len() - 1 {
//             print!(" - ");
//         }
//     }
// }
