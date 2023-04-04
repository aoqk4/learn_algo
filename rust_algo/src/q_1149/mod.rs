use std::{cmp::min, io::stdin};

pub fn q_1149() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let n = numbers[0_usize];

    let mut r = 0;
    let mut g = 0;
    let mut b = 0;

    let mut res: usize = 0;

    let mut arr: Vec<Vec<usize>> = vec![[0_usize; 3].to_vec(); 1001];

    for i in 1..n + 1 {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");

        let numbers: Vec<usize> = line
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        r = numbers[0_usize];
        g = numbers[1_usize];
        b = numbers[2_usize];

        arr[i][0] = min(arr[i - 1][1], arr[i - 1][2]) + r;
        arr[i][1] = min(arr[i - 1][0], arr[i - 1][2]) + g;
        arr[i][2] = min(arr[i - 1][0], arr[i - 1][1]) + b;
    }
    res = min(arr[n][2], min(arr[n][0], arr[n][1]));
    println!("{res}");
}
