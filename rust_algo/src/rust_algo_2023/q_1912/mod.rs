use std::cmp::max;
use std::io::stdin;

pub fn q_1912() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let n: usize = numbers[0_usize];

    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let arr: Vec<isize> = line
        .split_whitespace()
        .map(|num| num.parse::<isize>().unwrap())
        .collect();

    let mut res: Vec<isize> = arr.clone();

    if n == 1 {
        println!("{:?}", arr[0]);
        return;
    } else {
        for i in 1..n {
            res[i] = max(res[i], res[i - 1] + arr[i]);
        }
    }

    res.sort();

    println!("{:?}", res[res.len() - 1]);
}
