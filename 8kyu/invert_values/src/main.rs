// Given a set of numbers, return the additive inverse of each. Each positive becomes negatives,
// and the negatives become positives.
// invert([1,2,3,4,5]) == [-1,-2,-3,-4,-5]
// invert([1,-2,3,-4,5]) == [-1,2,-3,4,-5]
// invert([]) == []

#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    println!("Invert Values");
}

fn invert(values: &[i32]) -> Vec<i32> {
    values.iter().map(|x| -x).collect()

//  Or:
//    let mut result: Vec<i32> = vec![];
//    for i in values.iter() {
//        result.push(i * -1)
//    }
//    return result
}

#[cfg(test)]
mod test {
    use super::invert;

    #[test]
    fn basic_tests() {
        assert_eq!(invert(&vec![1,2,3,4,5]), vec![-1,-2,-3,-4,-5]);
        assert_eq!(invert(&vec![1,-2,3,-4,5]), vec![-1,2,-3,4,-5]);
        assert_eq!(invert(&vec![]), vec![]);
    }
}
