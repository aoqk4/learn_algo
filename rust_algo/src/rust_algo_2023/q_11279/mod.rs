use std::collections::BinaryHeap;
use std::fmt::Write;
use std::io::{stdin, stdout};

pub fn q_11279() {
    let stdout = stdout();
    let mut lock = stdout.lock();

    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let n = numbers[0_usize];

    let mut heap: BinaryHeap<usize> = BinaryHeap::with_capacity(100000);
    let mut output = String::new();

    for _ in 0..n {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");

        let num = line.trim().parse::<usize>().unwrap();

        match num {
            0 => {
                if heap.is_empty() {
                    writeln!(output, "{}", 0).unwrap();
                } else if !heap.is_empty() {
                    writeln!(output, "{}", heap.peek().unwrap()).unwrap();

                    heap.pop();
                }
            }
            _ => {
                heap.push(num);
            }
        }
    }
    print!("{}", output);
}
