// You might know some pretty large perfect squares. But what about the NEXT one?

// Complete the findNextSquare method that finds the next integral perfect square
// after the one passed as a parameter. Recall that an integral perfect square is an
// integer n such that sqrt(n) is also an integer.

// If the parameter is itself not a perfect square then -1 should be returned. You may
// assume the parameter is non-negative.

// Examples:(Input --> Output)

// 121 --> 144
// 625 --> 676
// 114 --> -1 since 114 is not a perfect square

pub fn find_next_square(sq: u64) -> Option<u64> {
    let root: f64 = (sq as f64).sqrt();
    match root == root.floor() {
        true => Some((root as u64 + 1).pow(2)),
        false => None,
    }
}

#[cfg(test)]
mod tests {
    use super::find_next_square;

    #[test]
    fn basic_tests() {
        assert_eq!(find_next_square(121), Some(144));
        assert_eq!(find_next_square(625), Some(676));
        assert_eq!(find_next_square(319_225), Some(320_356));
        assert_eq!(find_next_square(15_241_383_936), Some(15_241_630_849));
        assert_eq!(find_next_square(155), None);
        assert_eq!(find_next_square(342_786_627), None);
    }
}
