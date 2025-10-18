fn rotate_image(matrix: &mut Vec<Vec<i32>>) {
    let cloned = matrix.clone();
    let index = matrix.len() - 1;
    for i in 0..=index {
        for j in 0..=index {
            matrix[i][j] = cloned[index - j][i];
        }
    }
}

fn rotate_image_cool_but_complex(matrix: &mut Vec<Vec<i32>>) {
    let length = matrix.len();
    for i in 0..length {
        for j in 0..i {
            let tmp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = tmp;
        }
    }
    for i in 0..length {
        matrix[i].reverse();
    }
}

#[cfg(test)]
mod rotate_image_test {
    use super::*;

    #[test]
    fn rotate_image_test_1() {
        let mut input = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        rotate_image(&mut input);
        assert_eq!(input, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    }

    #[test]
    fn rotate_image_test_2() {
        let mut input = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        rotate_image(&mut input);
        assert_eq!(
            input,
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11]
            ]
        );
    }

    #[test]
    fn rotate_image_test_3() {
        let mut input = vec![vec![1]];
        rotate_image(&mut input);
        assert_eq!(input, vec![vec![1]]);
    }
}
