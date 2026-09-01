fn hamming_weight(n: u32) -> i32 {
    let mut res = 0;
    let mut n = n;
    for i in 0..32 {
        res += n & 1;
        n = n >> 1;
    }
    res as i32
}

fn hamming_weight_one_line(n: u32) -> i32 {
    n.count_ones() as i32
}

#[cfg(test)]
mod hamming_weight_test {
    use super::*;

    #[test]
    fn hamming_weight_test_1() {
        assert_eq!(hamming_weight(0b00000000000000000000000000010111), 4);
    }

    #[test]
    fn hamming_weight_test_2() {
        assert_eq!(hamming_weight(0b01111111111111111111111111111101), 30);
    }
}
