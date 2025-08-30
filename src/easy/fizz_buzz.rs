fn fizz_buzz(n: i32) -> Vec<String> {
    let mut res = vec![];
    for i in 1..=n {
        let new = match (i % 3 == 0, i % 5 == 0) {
            (true, true) => String::from("FizzBuzz"),
            (false, true) => String::from("Buzz"),
            (true, false) => String::from("Fizz"),
            _ => i.to_string(),
        };
        res.push(new);
    }
    res
}

#[cfg(test)]
mod fizz_buzz_test {
    use super::*;

    #[test]
    fn fizz_buzz_test_1() {
        assert_eq!(fizz_buzz(3), ["1", "2", "Fizz"]);
    }

    #[test]
    fn fizz_buzz_test_2() {
        assert_eq!(fizz_buzz(5), ["1", "2", "Fizz", "4", "Buzz"]);
    }

    #[test]
    fn fizz_buzz_test_3() {
        assert_eq!(
            fizz_buzz(15),
            [
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ]
        );
    }
}
