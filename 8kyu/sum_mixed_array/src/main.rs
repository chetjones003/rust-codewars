// Given an array of integers as strings and numbers, return the sum of the array values as if all
// were numbers.
// Return your answer as a number.

#![allow(dead_code)]
#![allow(unused_variables)]

use either::Either;

fn main() {
    println!("Sum Mixed Array");
}

fn sum_mix(arr: &[Either<i32, String>]) -> i32 {
    let mut result: i32 = 0;
    for element in arr {
        match element {
            Either::Left(x) => result += x,
            Either::Right(x) => result += x.parse::<i32>().unwrap(),
        }
    }
    return result
}

#[cfg(test)]
mod test {
    use super::sum_mix;
    use either::Either;

    fn dotest(arr: &[Either<i32, String>], expected: i32) {
        let actual = sum_mix(arr);
        assert!(actual == expected, "With arr = {arr:?}\nExpected: {expected}, Recieved: {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest(&[Either::Left(9), Either::Left(3), Either::Right("7".to_string()), Either::Right("3".to_string())], 22);
        dotest(&[Either::Right("5".to_string()), Either::Right("0".to_string().to_string()), Either::Left(9), Either::Left(3), Either::Left(2), Either::Left(1), Either::Right("9".to_string()), Either::Left(6), Either::Left(7)], 42);
        dotest(&[Either::Right("3".to_string()), Either::Left(6), Either::Left(6), Either::Left(0), Either::Right("5".to_string()), Either::Left(8), Either::Left(5), Either::Right("6".to_string()), Either::Left(2), Either::Right("0".to_string())], 41);
        dotest(&[Either::Right("1".to_string()), Either::Right("5".to_string()), Either::Right("8".to_string()), Either::Left(8), Either::Left(9), Either::Left(9), Either::Left(2), Either::Right("3".to_string())], 45);
        dotest(&[Either::Left(8), Either::Left(0), Either::Left(0), Either::Left(8), Either::Left(5), Either::Left(7), Either::Left(2), Either::Left(3), Either::Left(7), Either::Left(8), Either::Left(6), Either::Left(7)], 61);
    }
}
