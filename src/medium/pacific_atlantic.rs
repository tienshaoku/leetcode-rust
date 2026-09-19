fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use std::collections::VecDeque;
    let width = heights[0].len();
    let height = heights.len();

    let mut p_queue = VecDeque::new();
    let mut a_queue = VecDeque::new();
    for y in 0..height {
        p_queue.push_back((0, y));
        a_queue.push_back((width - 1, y));
    }
    for x in 0..width {
        p_queue.push_back((x, 0));
        a_queue.push_back((x, height - 1));
    }

    let mut pacific = vec![vec![false; width]; height];
    let mut atlantic = vec![vec![false; width]; height];
    while let Some((x, y)) = p_queue.pop_front() {
        if pacific[y][x] {
            continue;
        }

        pacific[y][x] = true;
        if x < width - 1 && heights[y][x + 1] >= heights[y][x] {
            p_queue.push_back((x + 1, y));
        }
        if y < height - 1 && heights[y + 1][x] >= heights[y][x] {
            p_queue.push_back((x, y + 1));
        }
        if x > 0 && heights[y][x - 1] >= heights[y][x] {
            p_queue.push_back((x - 1, y));
        }
        if y > 0 && heights[y - 1][x] >= heights[y][x] {
            p_queue.push_back((x, y - 1));
        }
    }

    let mut res = vec![];
    while let Some((x, y)) = a_queue.pop_front() {
        if atlantic[y][x] {
            continue;
        }

        atlantic[y][x] = true;
        if pacific[y][x] {
            res.push(vec![y as i32, x as i32]);
        }
        if x < width - 1 && heights[y][x + 1] >= heights[y][x] {
            a_queue.push_back((x + 1, y));
        }
        if y < height - 1 && heights[y + 1][x] >= heights[y][x] {
            a_queue.push_back((x, y + 1));
        }
        if x > 0 && heights[y][x - 1] >= heights[y][x] {
            a_queue.push_back((x - 1, y));
        }
        if y > 0 && heights[y - 1][x] >= heights[y][x] {
            a_queue.push_back((x, y - 1));
        }
    }
    res
}

#[cfg(test)]
mod pacific_atlantic_test {
    use super::*;
    use crate::vector::normalise;

    #[test]
    fn pacific_atlantic_test_1() {
        assert_eq!(
            normalise(pacific_atlantic(vec![
                vec![1, 2, 2, 3, 5],
                vec![3, 2, 3, 4, 4],
                vec![2, 4, 5, 3, 1],
                vec![6, 7, 1, 4, 5],
                vec![5, 1, 1, 2, 4]
            ])),
            [[0, 4], [1, 3], [1, 4], [2, 2], [3, 0], [3, 1], [4, 0]]
        );
    }

    #[test]
    fn pacific_atlantic_test_2() {
        assert_eq!(normalise(pacific_atlantic(vec![vec![1]])), [[0, 0]]);
    }

    #[test]
    fn pacific_atlantic_test_3() {
        assert_eq!(
            normalise(pacific_atlantic(vec![vec![1, 1], vec![1, 1], vec![1, 1]])),
            [[0, 0], [0, 1], [1, 0], [1, 1], [2, 0], [2, 1]]
        );
    }

    #[test]
    fn pacific_atlantic_test_4() {
        assert_eq!(
            normalise(pacific_atlantic(vec![
                vec![1, 2, 3],
                vec![8, 9, 4],
                vec![7, 6, 5]
            ])),
            [[0, 2], [1, 0], [1, 1], [1, 2], [2, 0], [2, 1], [2, 2]]
        );
    }
}
