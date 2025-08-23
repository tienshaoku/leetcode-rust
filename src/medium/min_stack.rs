struct MinStack {
    stack: Vec<(i32, i32)>,
    min: i32,
}

impl MinStack {
    fn new() -> Self {
        Self {
            stack: Vec::<(i32, i32)>::new(),
            min: i32::MAX,
        }
    }

    fn push(&mut self, val: i32) {
        self.min = self.min.min(val);
        self.stack.push((val, self.min));
    }

    fn pop(&mut self) {
        self.stack.pop();
        self.min = match self.stack.last() {
            Some(&(_, min)) => min,
            None => i32::MAX,
        };
    }

    // more graceful than self.stack.last().expect("err message")
    fn top(&self) -> i32 {
        match self.stack.last() {
            Some(&(v, _)) => v,
            None => i32::MAX,
        }
    }

    fn get_min(&self) -> i32 {
        self.min
    }
}

#[cfg(test)]
mod min_stack_test {
    use super::*;

    #[test]
    fn min_stack_test_1() {
        let mut stack = MinStack::new();
        stack.push(-2);
        stack.push(0);
        stack.push(-3);
        assert_eq!(stack.get_min(), -3);

        stack.pop();
        assert_eq!(stack.top(), 0);
        stack.get_min();
        assert_eq!(stack.get_min(), -2);
    }

    #[test]
    fn min_stack_test_2() {
        let mut stack = MinStack::new();
        stack.push(-2);
        assert_eq!(stack.top(), -2);
        assert_eq!(stack.get_min(), -2);

        stack.pop();
        stack.push(10);
        assert_eq!(stack.get_min(), 10);
        stack.push(-5);
        assert_eq!(stack.get_min(), -5);
    }
}
