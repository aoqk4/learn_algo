use std::fmt::Write;
// use std::collections::VecDeque;
use std::io::stdin;

pub fn q_1931() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let n = line.trim().parse::<usize>().unwrap();

    let mut target_vec:Vec<(usize, usize)> = vec![];

    for _ in 0..n {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");
    
        let numbers: Vec<usize> = line
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        target_vec.push((numbers[1_usize], numbers[0_usize]));
    }

    target_vec.sort();

    let mut cnt = 0;
    let mut t = 0;

    for item in 0..target_vec.len() {
        if t > target_vec[item].1 {
            continue;
        }
        cnt += 1;
        t = target_vec[item].0
    }
    println!("{:?}", cnt);

}