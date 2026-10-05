#[derive(Debug, Clone)]
struct Interval {
    pub start: i32,
    pub end: i32,
}

impl Interval {
    pub fn new(start: i32, end: i32) -> Self {
        Interval { start, end }
    }
}

fn meeting_rooms(intervals: Vec<Interval>) -> bool {
    if intervals.len() == 0 {
        return true;
    }
    let mut intervals = intervals;
    intervals.sort_by_key(|i| i.start);

    let mut max = intervals[0].end;
    for i in 1..intervals.len() {
        if intervals[i].start < max {
            return false;
        }
        max = max.max(intervals[i].end);
    }
    true
}

#[cfg(test)]
mod meeting_rooms_test {
    use super::*;

    #[test]
    fn meeting_rooms_test_1() {
        assert_eq!(
            meeting_rooms(vec![
                Interval::new(0, 30),
                Interval::new(5, 10),
                Interval::new(15, 20)
            ]),
            false
        );
    }

    #[test]
    fn meeting_rooms_test_2() {
        assert_eq!(
            meeting_rooms(vec![Interval::new(5, 8), Interval::new(9, 15)]),
            true
        );
    }

    #[test]
    fn meeting_rooms_test_3() {
        assert_eq!(
            meeting_rooms(vec![
                Interval::new(0, 20),
                Interval::new(30, 40),
                Interval::new(22, 27)
            ]),
            true
        );
    }

    #[test]
    fn meeting_rooms_test_4() {
        assert_eq!(
            meeting_rooms(vec![Interval::new(0, 8), Interval::new(8, 10)]),
            true
        );
    }

    #[test]
    fn meeting_rooms_test_5() {
        assert_eq!(
            meeting_rooms(vec![Interval::new(1, 5), Interval::new(1, 3)]),
            false
        );
    }

    #[test]
    fn meeting_rooms_test_6() {
        assert_eq!(
            meeting_rooms(vec![
                Interval::new(10, 15),
                Interval::new(20, 25),
                Interval::new(12, 17)
            ]),
            false
        );
    }
}
