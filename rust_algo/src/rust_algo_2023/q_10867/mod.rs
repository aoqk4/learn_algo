use std::fmt::Write;
use std::io::stdin;
use std::collections::BTreeSet;

fn q_10867() {
    let mut output = String::new();

    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let _n = line.trim().parse::<usize>().unwrap();

    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let target: BTreeSet<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    for item in target.iter() {
        write!(output, "{} ", item).unwrap();
    }

    println!("{}", output);
}