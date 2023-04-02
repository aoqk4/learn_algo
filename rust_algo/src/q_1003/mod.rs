// Todo : Use HashMap<> !!!!!!!!!!!!!!!!
use std::io::stdin;
pub fn q_1003() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let t: usize = numbers[0_usize];

    let mut res: Vec<usize> = vec![0; 41];

    res[1] = 1;

    for i in 2..res.len() {
        res[i] = res[i - 1] + res[i - 2];
    }

    for _ in 0..t {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");

        let numbers: Vec<usize> = line
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        let n: usize = numbers[0_usize];

        if n == 0 {
            println!("1 0");
        } else if n == 1 {
            println!("0 1");
        } else {
            println!("{} {}", res[n - 1], res[n]);
        }
    }
}
