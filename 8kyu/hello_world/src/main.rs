#![allow(dead_code)]
#![allow(unused_variables)]
fn main() {
}

fn sim_stdin() -> String {
    String::from("hello world!")
}

fn greet() -> String {
    sim_stdin()
}

#[cfg(test)]
mod test {
    use super::greet;

    fn test_greet_the_world() {
        assert_eq!(greet(), "hello world!", "should return the correct message");
    }
}
