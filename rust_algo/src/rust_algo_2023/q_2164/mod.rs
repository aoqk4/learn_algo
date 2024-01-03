// fail...... Time Over
use std::io::stdin;
use std::io::{stdout, Write};

#[derive(Debug, Clone)]
struct CircularBuffer<T> {
    buffer: Vec<Option<T>>,
    head: usize,
    tail: usize,
}

impl<T> CircularBuffer<T> {
    fn new(size: usize) -> Self {
        CircularBuffer {
            buffer: vec![None; size],
            head: 0,
            tail: 0,
        }
    }

    fn enqueue(&mut self, item: T) {
        if self.buffer[self.tail].is_none() {
            self.buffer[self.tail] = Some(item);
            self.tail = (self.tail + 1) % self.buffer.len();
        } else {
            panic!("Circular buffer overflow");
        }
    }

    fn dequeue(&mut self) -> Option<T> {
        if let Some(item) = self.buffer[self.head].take() {
            self.head = (self.head + 1) % self.buffer.len();
            Some(item)
        } else {
            None
        }
    }

    fn length(&mut self) -> usize {
        self.buffer.len()
    }
}
pub fn q_2164() {
    let stdout = stdout();
    let mut lock = stdout.lock();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();

    let mut arr: CircularBuffer<usize> = CircularBuffer::new(n);

    for i in 1..n + 1 {
        arr.enqueue(i);
    }

    println!("{:?}", arr);

    loop {
        if arr.length() == 1 {
            break;
        }
        arr.dequeue();
        arr.enqueue(arr.buffer[0].unwrap());
    }

    writeln!(lock, "{:?}", arr.buffer[0]).unwrap();
}
