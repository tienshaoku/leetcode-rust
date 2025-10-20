fn highest_altitude(gain: Vec<i32>) -> i32 {
    gain.iter()
        .fold((0, 0), |(max, sum), &x| (max.max(sum + x), sum + x))
        .0
}

fn highest_altitude_lengthy_but_saves_memory(gain: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut sum = 0;
    for i in gain {
        sum += i;
        if sum > max {
            max = sum;
        }
    }
    max
}

#[cfg(test)]
mod highest_altitude_test {
    use super::*;

    #[test]
    fn highest_altitude_test_1() {
        assert_eq!(highest_altitude(vec![-5, 1, 5, 0, -7]), 1);
    }

    #[test]
    fn highest_altitude_test_2() {
        assert_eq!(highest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]), 0);
    }
}
