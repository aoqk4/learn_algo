use std::io::stdin;

pub fn q_2217() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let n: usize = numbers[0_usize];

    let mut arr: Vec<usize> = Vec::with_capacity(100001);

    let mut res = 0;

    for _ in 0..n {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");

        let numbers: Vec<usize> = line
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        arr.push(numbers[0_usize]);
    }

    arr.sort();

    for i in 0..n {
        let sum: usize = arr[i] * (n - i);

        if sum > res {
            res = sum;
        }
    }

    println!("{res}");
}
