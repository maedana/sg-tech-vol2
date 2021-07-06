fn main() {
    for i in 1..=20 {
        println!("{}", fizzbuzz(i));
    }
}

fn fizzbuzz(number: i32) -> String {
    if number % 3 == 0 && number % 5 == 0 {
        return "FizzBuzz".to_string();
    }
    if number % 3 == 0 {
        return "Fizz".to_string();
    }

    if number % 5 == 0 {
        return "Buzz".to_string();
    }

    return format!("{}", number);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_fizz() {
        assert_eq!(fizzbuzz(3), "Fizz");
    }

    #[test]
    fn print_buzz() {
        assert_eq!(fizzbuzz(5), "Buzz");
    }

    #[test]
    fn print_fizzbuzz() {
        assert_eq!(fizzbuzz(15), "FizzBuzz");
    }

    #[test]
    fn print_number() {
        assert_eq!(fizzbuzz(1), "1");
    }
}
