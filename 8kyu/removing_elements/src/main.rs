// Take an array and remove every second element from the array. Always keep the first element and
// start removing with the next element.
//
// Example:
// ["Keep", "Remove", "Keep", "Remove", "Keep", ...] --> ["Keep", "Keep", "Keep", ...]
//
// None of the arrays will be empty, so you don't have to worry about that!

#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    println!("Removing Elements");
}

fn remove_every_other(arr: &[u8]) -> Vec<u8> {
    return arr.to_vec().iter().step_by(2).cloned().collect()
}

#[cfg(test)]
mod test {
    use super::remove_every_other;

    #[test]
    fn basic_tests() {
        assert_eq!(remove_every_other(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), &[1, 3, 5, 7, 9]);
    }
}
