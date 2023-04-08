use std::fmt::Write;
use std::{cmp::Ordering, io::stdin};

pub fn q_11650() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let n = numbers[0_usize];

    let mut arr: Vec<(isize, isize)> = vec![];

    for _ in 0..n {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");

        let numbers: Vec<isize> = line
            .split_whitespace()
            .map(|num| num.parse::<isize>().unwrap())
            .collect();

        arr.push((numbers[0_usize], numbers[1_usize]));
    }

    arr.sort_by(|a, b| {
        if a.0.cmp(&b.0) == Ordering::Equal {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });

    let mut output = String::new();

    for i in 0..arr.len() {
        writeln!(output, "{} {}", arr[i].0, arr[i].1).unwrap();
    }

    print!("{}", output);
}
