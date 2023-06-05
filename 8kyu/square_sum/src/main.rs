// Complete the square sum function so that it squares each number passed into it and then sums the
// results together.
// [1, 2, 2] = 1^2 + 2^2 + 2^2 = 9

#![allow(dead_code)]
#![allow(unused_variables)]
fn main() {
    println!("Square(n) Sum Kata");
}

fn square_sum(vec: Vec<i32>) -> i32 {
    vec.iter().map(|x| x.pow(2)).sum::<i32>()
}

#[cfg(test)]
mod test {
    use super::square_sum;

    #[test]
    fn returns_expected() {
        assert_eq!(square_sum(vec![1, 2]), 5);
        assert_eq!(square_sum(vec![5, 3, 4]), 50);
        assert_eq!(square_sum(vec![-1, -2]), 5);
        assert_eq!(square_sum(vec![1, 0, 1]), 2);
        assert_eq!(square_sum(vec![-1, 1]), 2);
        assert_eq!(square_sum(vec![]), 0);
    }
    
}
