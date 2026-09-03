fn sum_of_two_integer(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        // carry handles the carrying
        let carry = a & b;
        // a handles the sum, excluding carrying ones
        a ^= b;
        // carry propagates
        b = carry << 1;
    }
    a
}

fn sum_of_two_integer_lengthy(a: i32, b: i32) -> i32 {
    let a_arr = format!("{:032b}", a)
        .chars()
        .map(|c| c.to_digit(2).unwrap())
        .collect::<Vec<u32>>();
    let b_arr = format!("{:032b}", b)
        .chars()
        .map(|c| c.to_digit(2).unwrap())
        .collect::<Vec<u32>>();
    let mut res = vec![0; 32];
    let mut prop = false;
    for i in (0..32).rev() {
        if prop {
            res[i] = (a_arr[i] ^ b_arr[i]) ^ 1;
            prop = (a_arr[i] | b_arr[i]) != 0;
        } else {
            res[i] = a_arr[i] ^ b_arr[i];
            prop = (a_arr[i] & b_arr[i]) != 0;
        }
    }
    res.iter().fold(0u32, |acc, &bit| (acc << 1) | bit) as i32
}

#[cfg(test)]
mod sum_of_two_integer_test {
    use super::*;

    #[test]
    fn sum_of_two_integer_test_1() {
        assert_eq!(sum_of_two_integer(1, 2), 3);
    }

    #[test]
    fn sum_of_two_integer_test_2() {
        assert_eq!(sum_of_two_integer(2, 3), 5);
    }

    #[test]
    fn sum_of_two_integer_test_3() {
        assert_eq!(sum_of_two_integer(20, 30), 50);
    }
}
