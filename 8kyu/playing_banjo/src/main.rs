// Create a function which answers the question "Are you playing banjo?".
// If your name starts with the letter "R" or lower case "r", you are playing banjo!
//
// The function takes a name as its only argument, and returns one of the following strings:
//
// name + " plays banjo" 
// name + " does not play banjo"

#![allow(dead_code)]
#![allow(unused_variables)]

use core::panic;

fn main() {
    println!("Hello, world!");
}

fn are_you_playing_banjo(name: &str) -> String {
    let first_letter: char = name[0..1].parse().unwrap();
    if name.is_empty() {
        panic!("Empty String")
    } else {
        match first_letter {
            'r' | 'R' => format!("{} plays banjo", name),
            'a'..='q' | 's'..='z' | 'A'..='Q' | 'S'..='Z' => {
                format!("{} does not play banjo", name)
            },
            '0'..='9' => panic!("First letter is a number"),
            _ => panic!("No special symbols allowed"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::are_you_playing_banjo;

    #[test]
    fn basic_tests() {
        assert_eq!(are_you_playing_banjo("Martin"), "Martin does not play banjo");
        assert_eq!(are_you_playing_banjo("Rikke"), "Rikke plays banjo");
        assert_eq!(are_you_playing_banjo("bravo"), "bravo does not play banjo");
        assert_eq!(are_you_playing_banjo("rolf"), "rolf plays banjo");
    }
}
