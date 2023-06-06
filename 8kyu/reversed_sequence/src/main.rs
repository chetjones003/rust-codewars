// Build a function that returns an array of integers from n to 1 where n>0.
//
// Example : n=5 --> [5,4,3,2,1]

#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    println!("Hello, world!");
}

fn reverse_seq(n: u32) -> Vec<u32> {
    let mut result = Vec::new();
    let mut i = n;
    while i > 0 {
        result.push(i);
        i -= 1
    }
    return result

    // Cool Solution
    // return (1..=n).rev().collect()
}

#[cfg(test)]
mod test {
    use super::reverse_seq;

    #[test]
    fn basic_tests() {
        assert_eq!(reverse_seq(5), [5, 4, 3, 2, 1].to_vec());
    }
}
