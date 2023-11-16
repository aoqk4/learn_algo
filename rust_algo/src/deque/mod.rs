#[derive(Debug, Clone, Copy)]
pub struct Deque<T> {
    pub data: [T; 150001],
    tail: usize,
    head: usize,
}

impl Default for Deque<usize> {
    fn default() -> Self {
        Self { data: [0; 150001], tail: 150000 / 2, head: 150000 / 2 }
    }
}

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

impl Move<usize> for Deque<usize> {
    fn push_front(&mut self, x:usize) {
        self.head += 1;
        self.data[self.head + 1] = x;
    }

    fn front(&mut self) -> usize {
        self.data[self.head]
    }

    fn push_back(&mut self, x:usize) {
        self.data[self.tail] = x;
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
        // let deque = Deque::default();
        // deque.push_front(30);
        assert_eq!(75000, 150000 / 2);
    }
}