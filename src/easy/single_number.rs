// xor properties:
// 1. same elements cancel off each other: a ^ a = 0; a ^ 0 = a
// 2. in chained xor calculations, the order doesn't matter: a ^ b ^ c = c ^ b ^ a

fn single_number(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |accu, current| accu ^ current)
}

#[cfg(test)]
mod single_number_test {
    use super::*;

    #[test]
    fn single_number_test_1() {
        assert_eq!(single_number(vec![2, 2, 1]), 1);
    }

    #[test]
    fn single_number_test_2() {
        assert_eq!(single_number(vec![1]), 1);
    }

    #[test]
    fn single_number_test_3() {
        assert_eq!(single_number(vec![4, 1, 2, 1, 2]), 4);
    }
}
