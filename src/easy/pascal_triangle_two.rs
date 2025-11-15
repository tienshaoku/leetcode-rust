fn pascal_triangle_two(row_index: i32) -> Vec<i32> {
    let row_index = row_index as usize;
    let mut arr = vec![1];
    for i in 1..=row_index {
        let mut prev = 1;
        let mut tmp = 0;
        for j in 1..=i {
            if j == i {
                arr.push(1);
                continue;
            }
            tmp = arr[j];
            arr[j] += prev;
            prev = tmp;
        }
    }
    arr
}

#[cfg(test)]
mod pascal_triangle_two_test {
    use super::*;

    #[test]
    fn pascal_triangle_two_test_1() {
        assert_eq!(pascal_triangle_two(3), [1, 3, 3, 1]);
    }

    #[test]
    fn pascal_triangle_two_test_2() {
        assert_eq!(pascal_triangle_two(0), [1]);
        assert_eq!(pascal_triangle_two(1), [1, 1]);
    }
}
