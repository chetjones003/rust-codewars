// In this game, the hero moves from left to right. The player rolls the dice and moves the number
// of spaces indicated by the dice two times.
//
// Create a function for the terminal game that takes the current position of the hero and the roll
// (1-6) and return the new position.

#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    println!("Hello, world!");
}

fn move_hero(position: u32, roll: u32) -> u32 {
    return position + (roll * 2)
}

#[cfg(test)]
mod test {
    use super::move_hero;

    #[test]
    fn basic_test() {
        assert_eq!(move_hero(0, 4), 8);
        assert_eq!(move_hero(3, 6), 15);
    }
}
