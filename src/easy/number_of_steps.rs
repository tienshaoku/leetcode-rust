fn number_of_steps(num: i32) -> i32 {
    let bits = format!("{:b}", num);
    (bits.matches('1').count() + bits.len()) as i32 - 1
}

#[cfg(test)]
mod number_of_steps_test {
    use super::*;

    #[test]
    fn number_of_steps_test_1() {
        assert_eq!(number_of_steps(14), 6);
    }

    #[test]
    fn number_of_steps_test_2() {
        assert_eq!(number_of_steps(8), 4);
    }

    #[test]
    fn number_of_steps_test_3() {
        assert_eq!(number_of_steps(123), 12);
    }
}
