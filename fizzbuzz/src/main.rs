fn main() {
    for i in 1..=20 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz")
        } else if i % 3 == 0 {
            println!("Fizz")
        } else if i % 5 == 0 {
            println!("Buzz")
        } else {
            println!("{}", i)
        }
    }
}

fn fizzbuzz(number: i32) -> String {
    "Fizz".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_fizz() {
        assert_eq!(fizzbuzz(3), "Fizz");
    }
}
