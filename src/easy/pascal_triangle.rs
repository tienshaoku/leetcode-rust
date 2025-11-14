fn pascal_triangle(num_rows: i32) -> Vec<Vec<i32>> {
    let mut res = vec![vec![1]];
    for i in 1..num_rows as usize {
        let mut arr = vec![];
        for j in 0..=i {
            if j == 0 || j == i {
                arr.push(1);
            } else {
                arr.push(res[i - 1][j - 1] + res[i - 1][j]);
            }
        }
        res.push(arr);
    }
    res
}

#[cfg(test)]
mod pascal_triangle_test {
    use super::*;

    #[test]
    fn pascal_triangle_test_1() {
        assert_eq!(
            pascal_triangle(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }

    #[test]
    fn pascal_triangle_test_2() {
        assert_eq!(pascal_triangle(1), vec![vec![1]]);
    }
}
