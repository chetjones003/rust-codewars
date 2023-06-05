// You're writing code to control your town's traffic lights. You need a function to handle each
// change from green, to yellow, to red, and then to green again.
//
// Complete the function that takes a string as an argument representing the current state of the
// light and returns a string representing the state the light should change to.
//
// For example, when the input is green, output should be yellow.

#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    println!("Hello, world!");
}

fn update_light(current: &str) -> String {
    match current {
        "green" => "yellow",
        "yellow" => "red",
        "red" => "green",
        _ => panic!("the light's broke"),
    }.into()
}

#[cfg(test)]
mod test {
    use super::update_light;

    fn basic_test() {
        assert_eq!(update_light("green"), "yellow");
        assert_eq!(update_light("yellow"), "red");
        assert_eq!(update_light("red"), "green");
    }
}
