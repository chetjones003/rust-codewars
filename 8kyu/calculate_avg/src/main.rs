// Write a function which calculates the average of the numbers in a given list.
//
// Note: Empty arrays should return 0.

#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    println!("Calculate Average");
}

fn find_avg(slice: &[f64]) -> f64 {
    match slice.len() {
        0 => 0.0,
        size => slice.iter().sum::<f64>() / size as f64
    }
}

#[cfg(test)]
mod test {
    use super::find_avg;
    use float_eq::assert_float_eq;

    #[test]
    fn example() {
        let input = [17.0, 16.0, 16.0, 16.0, 16.0, 15.0, 17.0, 17.0, 15.0, 5.0, 17.0, 17.0, 16.0,];
        assert_float_eq!(
            find_avg(&input),
            200.0 / 13.0,
            abs <= 1e-9, r2nd <= 4.0 * f64::EPSILON
        );
        assert_float_eq!(find_avg(&[]), 0.0, abs <= 1e-9, r2nd <= 4.0 * f64::EPSILON);
    }
}
