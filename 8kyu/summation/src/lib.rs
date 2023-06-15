// Write a program that finds the summation of every number from 1 to num.
// The number will always be a positive integer greater than 0.

// For example (Input -> Output):
//
// 2 -> 3 (1 + 2)
// 8 -> 36 (1 + 2 + 3 + 4 + 5 + 6 + 7 + 8)

pub fn summation(n: i32) -> i32 {
    let mut result: i32 = 0;
    for index in 0..=n {
        result = result + index
    }
    result
}

#[cfg(test)]
mod test {
    use super::summation;

    #[test]
    fn basic_tests() {
        assert_eq!(summation(1), 1);
        assert_eq!(summation(8), 36);
        assert_eq!(summation(22), 253);
        assert_eq!(summation(100), 5050);
        assert_eq!(summation(213), 22791);
    }
}
