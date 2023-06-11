#![allow(dead_code)]
#![allow(unused_variables)]

// Create a function with two arguments that will return an array of the first n multiples of x.
//
// Assume both the given number and the number of times to count will be positive numbers greater
// than 0.
//
// Return the results as an array or list ( depending on language ).
// Examples
//
// count_by(1, 10) // should return vec![1,2,3,4,5,6,7,8,9,10]
// count_by(2,5) // should return vec![2,4,6,8,10]

fn main() {
    count_by(2, 5);
}

fn count_by(x: u32, n: u32) -> Vec<u32> {
    // println!("Input: ({x}, {n})");
    let mut elements: Vec<u32> = Vec::new();
    let mut i = 1;
    while i <= n {
        if elements.len() < 1 {
            // println!("1. i={i}, x={x}");
            elements.push(x)
        } else {
            // println!("2. i={i}, x={x}");
            elements.push(i * x)
        }
        i += 1;
    }
    // println!("Output: {:?}", elements);
    return elements

    // Cool solution *dfhwze,won*
    // (1..=n).map(|e| x * e).collect()
}

#[cfg(test)]
mod test {
    use super::count_by;
    use itertools::Itertools;

    #[test]
    fn sample_tests() {
        assertion(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], (1, 10));
        assertion(vec![2, 4, 6, 8, 10], (2, 5));
        assertion(vec![3, 6, 9, 12, 15, 18, 21], (3, 7));
        assertion(vec![50, 100, 150, 200, 250], (50, 5));
        assertion(vec![100, 200, 300, 400, 500, 600], (100, 6));
    }

    fn assertion(expected: Vec<u32>, inputs: (u32, u32)) {
        let actual = count_by(inputs.0, inputs.1);

        assert!(
            expected == actual,
            "\nTest failed!\n expected: [{}]\n actual: [{}]\n x: {}\n n: {}\n",
            expected.iter().join(", "),
            actual.iter().join(", "),
            inputs.0,
            inputs.1
        );
    }
}
