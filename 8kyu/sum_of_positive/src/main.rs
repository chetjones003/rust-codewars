#![allow(dead_code)]
#![allow(unused_variables)]

// You get an array of numbers, return the sum of all of the positives ones.
//
// Example [1,-4,7,12] => 1 + 7 + 12 = 20
//
// Note: if there is nothing to sum, the sum is default to 0.

fn main() {
    println!("Hello, world!");
}

fn positive_sum(slice: &[i32]) -> i32 {
    return slice.iter().filter(|x| x.is_positive()).sum()
}

#[cfg(test)]
mod test {
    use super::positive_sum;

    #[test]
    fn basic_test() {
        assert_eq!(positive_sum(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(positive_sum(&[1, -2, 3, 4, 5]), 13);
        assert_eq!(positive_sum(&[-1, 2, 3, 4, -5]), 9);
    }

    #[test]
    fn empty_list() {
        assert_eq!(positive_sum(&[]), 0);
    }

    #[test]
    fn all_negative() {
        assert_eq!(positive_sum(&[-1, -2, -3, -4, -5]), 0);
    }
}
