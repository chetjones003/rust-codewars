// When provided with a number between 0-9, return it in words.
//
// Input :: 1
//
// Output :: "One".

#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    println!("Switch it up");
}

fn switch_it_up(n: usize) -> &'static str {
    const WORDS: [&str; 10] = ["Zero", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine"];
    match n {
        0..=9 => WORDS[n],
        _ => panic!("Enter a number between 0 and 9..."),
    }
}

#[cfg(test)]
mod test {
    use super::switch_it_up;

    #[test]
    fn basic() {
        assert_eq!(switch_it_up(1), "One");
        assert_eq!(switch_it_up(2), "Two");
        assert_eq!(switch_it_up(3), "Three");
        assert_eq!(switch_it_up(4), "Four");
        assert_eq!(switch_it_up(5), "Five");
        assert_eq!(switch_it_up(6), "Six");
        assert_eq!(switch_it_up(7), "Seven");
        assert_eq!(switch_it_up(8), "Eight");
        assert_eq!(switch_it_up(9), "Nine");
    }
}
