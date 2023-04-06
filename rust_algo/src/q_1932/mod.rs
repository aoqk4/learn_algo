use std::cmp::max;
use std::io::stdin;

pub fn q_1932() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let n = numbers[0_usize];

    let mut arr: Vec<Vec<usize>> = vec![];

    for _ in 0..n {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");

        let numbers: Vec<usize> = line
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        arr.push(numbers);
    }

    let mut res: usize = arr[0][0];

    for i in 1..n {
        for j in 0..i + 1 {
            if j == 0 {
                arr[i][j] = arr[i - 1][0] + arr[i][j];
            } else if i == j {
                arr[i][j] = arr[i - 1][j - 1] + arr[i][j];
            } else {
                arr[i][j] = max(arr[i - 1][j - 1] + arr[i][j], arr[i - 1][j] + arr[i][j]);
            }

            res = max(res, arr[i][j]);
        }
    }

    println!("{:?}", res);
}
