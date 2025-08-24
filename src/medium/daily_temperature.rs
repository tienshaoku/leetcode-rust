fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut res = vec![0; temperatures.len()];
    let mut arr = vec![];

    for i in 0..temperatures.len() {
        while let Some(&last) = arr.last() {
            if temperatures[i] > temperatures[last] {
                res[last] = (i - last) as i32;
                arr.pop();
            } else {
                break;
            }
        }
        arr.push(i);
    }
    res
}

fn daily_temperatures_fat(temperatures: Vec<i32>) -> Vec<i32> {
    let mut res = vec![0; temperatures.len()];
    let mut arr: Vec<(i32, usize)> = vec![];

    for i in 0..temperatures.len() {
        while arr.len() > 0 && temperatures[i] > arr[arr.len() - 1].0 {
            let (_, index) = arr[arr.len() - 1];
            res[index] = (i - index) as i32;
            arr.pop();
        }
        arr.push((temperatures[i], i));
    }
    res
}

// [73, 74, 75, 71, 69, 72, 76, 73]
// [1,  1,  4,  2,  1,  1,  0,  0]

// [55,38,53,81,61,93,97,32,43,78]
// [3, 1, 1, 2, 1, 1, 0, 1, 1, 0]

// [30, 40, 50, 60]
// [1,  1,  1,  0]

#[cfg(test)]
mod daily_temperatures_test {
    use super::*;

    #[test]
    fn daily_temperatures_test_1() {
        assert_eq!(
            daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }

    #[test]
    fn daily_temperatures_test_2() {
        assert_eq!(daily_temperatures(vec![30, 40, 50, 60]), vec![1, 1, 1, 0]);
    }

    #[test]
    fn daily_temperatures_test_3() {
        assert_eq!(daily_temperatures(vec![30, 60, 90]), vec![1, 1, 0]);
    }

    #[test]
    fn daily_temperatures_test_4() {
        assert_eq!(
            daily_temperatures(vec![55, 38, 53, 81, 61, 93, 97, 32, 43, 78]),
            vec![3, 1, 1, 2, 1, 1, 0, 1, 1, 0]
        );
    }
}
