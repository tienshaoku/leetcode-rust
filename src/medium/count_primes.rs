fn count_primes(n: i32) -> i32 {
    if n < 2 {
        return 0;
    }

    let n = n as usize;
    let mut vec = vec![true; n];
    let mut i = 2;
    while i * i <= n {
        if vec[i] {
            // the first unmarked prime will be i * i
            for j in (i * i..n).step_by(i) {
                vec[j] = false;
            }
        }
        i += 1;
    }
    // skip 1 & 2
    vec.iter().skip(2).map(|v| *v as i32).sum()
}

fn count_primes_slow(n: i32) -> i32 {
    if n < 3 {
        return 0;
    }

    let max = (n as f64).sqrt() as i32 + 1;
    let mut vec = vec![2];
    let mut is_not_prime = false;
    for i in 2..n {
        for j in &vec {
            if *j > max {
                break;
            }
            if i % j == 0 {
                is_not_prime = true;
                break;
            }
        }
        if !is_not_prime {
            vec.push(i);
        } else {
            is_not_prime = false;
        }
    }
    vec.len() as i32
}

#[cfg(test)]
mod count_primes_test {
    use super::*;

    #[test]
    fn count_primes_test_1() {
        assert_eq!(count_primes(10), 4);
    }

    #[test]
    fn count_primes_test_2() {
        assert_eq!(count_primes(0), 0);
        assert_eq!(count_primes(1), 0);
        assert_eq!(count_primes(2), 0);
        assert_eq!(count_primes(3), 1);
    }

    #[test]
    fn count_primes_test_3() {
        // 2,3,5,7,11,13,17,19
        assert_eq!(count_primes(20), 8);
    }
}
