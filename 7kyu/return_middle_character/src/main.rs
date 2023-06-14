#![allow(dead_code)]
#![allow(unused_variables)]

// You are going to be given a word. Your job is to return the middle character of the word. If the
// word's length is odd, return the middle character. If the word's length is even, return the
// middle 2 characters.

fn main() {
    println!("{}", get_middle("tested"));
}

fn get_middle(s: &str) -> &str {
    let length: usize = s.len();
    let start: usize = (length - 1) / 2;
    let end: usize = (length / 2) + 1;
    return &s[start..end];
}

#[cfg(test)]
mod test {
    use super::get_middle;

    #[test]
    fn basic_tests() {
        assert_eq!(get_middle("test"), "es");
        assert_eq!(get_middle("testing"), "t");
        assert_eq!(get_middle("middle"), "dd");
        assert_eq!(get_middle("A"), "A");
        assert_eq!(get_middle("of"), "of");
    }
}
