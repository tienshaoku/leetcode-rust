fn palindrome_number(x: i32) -> bool {
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }

    let mut original = x;
    let mut reversed = 0;
    while original > reversed {
        reversed = reversed * 10 + original % 10;
        original /= 10;
    }

    original == reversed || reversed / 10 == original
}

fn palindrome_number_slow(x: i32) -> bool {
    match x {
        ..0 => return false,
        0..10 => return true,
        _ => (),
    }

    let mut power = (x as f32).log10().floor() as i32;
    let mut clone = x;
    while power > 0 {
        let multiplier = 10_i32.pow(power as u32);
        let min = clone % 10;
        let max = clone / multiplier;
        if min != max {
            return false;
        }
        clone -= max * multiplier;
        clone /= 10;
        power -= 2;
    }
    true
}

#[cfg(test)]
mod palindrome_number_test {
    use super::*;

    #[test]
    fn palindrome_number_test_1() {
        assert_eq!(palindrome_number(121), true);
    }

    #[test]
    fn palindrome_number_test_2() {
        assert_eq!(palindrome_number(-1), false);
    }

    #[test]
    fn palindrome_number_test_3() {
        assert_eq!(palindrome_number(10), false);
    }

    #[test]
    fn palindrome_number_test_4() {
        assert_eq!(palindrome_number(11), true);
    }

    #[test]
    fn palindrome_number_test_5() {
        assert_eq!(palindrome_number(3223223), true);
    }
}
