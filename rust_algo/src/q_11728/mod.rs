// Fail to runtime Err (stdin Buffer size)
use std::fmt::Write;
use std::io::stdin;
pub fn q_11728() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let n = numbers[0_usize];
    let m = numbers[1_usize];

    // A
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let a: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    // B
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let b: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let mut a_cnt: usize = 0;
    let mut b_cnt: usize = 0;
    let mut output = String::new();

    loop {
        if a_cnt >= n || b_cnt >= m {
            break;
        }
        if a[a_cnt] > b[b_cnt] {
            write!(output, "{} ", b[b_cnt]).unwrap();
            b_cnt += 1;
        } else if a[a_cnt] < b[b_cnt] {
            write!(output, "{} ", a[a_cnt]).unwrap();
            a_cnt += 1;
        }
    }

    for i in a_cnt..n {
        write!(output, "{} ", a[i]).unwrap();
    }

    for i in b_cnt..m {
        write!(output, "{} ", b[i]).unwrap();
    }

    println!("{}", output);
}
