fn decode_ways(s: String) -> i32 {
    if s.starts_with('0') {
        return 0;
    }
    let vals = s
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect::<Vec<i32>>();
    let mut arr = vec![0; s.len()];
    arr[0] = 1;

    for i in 1..vals.len() {
        let now = vals[i];
        let prev = vals[i - 1];
        let mut current = 0;

        // use all combos of arr[i - 1] appending now
        if now != 0 {
            current += arr[i - 1];
        }

        let pair_success = match prev {
            1 => true,
            2 => now < 7,
            _ => false,
        };
        // take (prev,now) as a pair, appending to all combos of arr[i - 2]
        if pair_success {
            current += if i >= 2 { arr[i - 2] } else { 1 };
        }

        arr[i] = current;
    }
    *arr.last().unwrap()
}

fn decode_ways_time_exceeded_but_explicit(s: String) -> i32 {
    if s.starts_with('0') {
        return 0;
    }
    let vals = s
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect::<Vec<i32>>();
    let mut arr = vec![vec![vals[0]]];
    for i in 1..vals.len() {
        let now = vals[i];
        let mut current = vec![];
        for p in arr {
            if now != 0 {
                let mut first = p.clone();
                first.push(now);
                current.push(first);
            }
            match p.last().unwrap() {
                1 => {
                    let mut second = p.clone();
                    let length = second.len();
                    second[length - 1] = 10 + now;
                    current.push(second);
                }
                2 => {
                    if now < 7 {
                        let mut second = p.clone();
                        let length = second.len();
                        second[length - 1] = 10 + now;
                        current.push(second);
                    }
                }
                _ => (),
            }
        }
        arr = current;
    }
    arr.len() as i32
}

#[cfg(test)]
mod decode_ways_test {
    use super::*;

    #[test]
    fn decode_ways_test_1() {
        assert_eq!(decode_ways(String::from("12")), 2);
    }

    #[test]
    fn decode_ways_test_2() {
        assert_eq!(decode_ways(String::from("01")), 0);
    }

    #[test]
    fn decode_ways_test_3() {
        assert_eq!(decode_ways(String::from("1")), 1);
    }

    #[test]
    fn decode_ways_test_4() {
        assert_eq!(decode_ways(String::from("1012")), 2);
    }

    #[test]
    fn decode_ways_test_5() {
        assert_eq!(decode_ways(String::from("226")), 3);
    }

    #[test]
    fn decode_ways_test_6() {
        assert_eq!(decode_ways(String::from("12")), 2);
    }

    #[test]
    fn decode_ways_test_7() {
        assert_eq!(decode_ways(String::from("2101")), 1);
    }

    #[test]
    fn decode_ways_test_8() {
        assert_eq!(decode_ways(String::from("1212121212121212121212121212121212121212121277777777777777777777777777777777777777777777777777777777")), 1134903170);
    }
}
