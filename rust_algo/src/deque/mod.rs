trait Move<T> {
    fn push_front(&mut self, x:T) {}
    fn push_back(&mut self, x:T) {}
    fn pop_front(&mut self) {}
    fn pop_back(&mut self) {}
    fn front(&mut self) -> T {
        todo!()
    }
    fn back(&mut self) -> T {
        todo!()
    }
}


#[derive(Debug, Clone, Copy, Default)]
pub struct Deque<T> {
    pub data: [T; 32],
    pub tail: usize,
    pub head: usize,
}

impl From<[usize; 32]> for Deque<usize> {
    fn from(value: [usize; 32]) -> Self {
        Self { data: value, tail: 32 / 2, head: 32 / 2 }
    }
}

impl Move<usize> for Deque<usize> {
    fn push_front(&mut self, x:usize) {
        self.head -= 1;
        self.data[self.head] = x;
    }
    fn pop_front(&mut self) {
        self.head += 1;

    }
    fn front(&mut self) -> usize {
        self.data[self.head]
    }

    fn push_back(&mut self, x:usize) {
        self.tail += 1;
        self.data[self.tail] = x;
    }
    fn pop_back(&mut self) {
        self.tail -=1;
    }
    fn back(&mut self) -> usize {
        self.data[self.tail]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn front_push_case() {
        let mut deque = Deque::from([0_usize; 32]);
        deque.push_front(30);
        assert_eq!(30, deque.front());
    }
}