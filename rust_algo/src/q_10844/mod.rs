use std::io::stdin;

pub fn q_10844() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let n: usize = numbers[0_usize];

    const DIV: usize = 1000000000;

    let mut arr: Vec<Vec<usize>> = vec![[0_usize; 10].to_vec(); 101];

    let mut res: usize = 0;

    for i in 1..10 {
        arr[1][i] = 1;
    }

    for i in 2..n + 1 {
        for j in 0..10 {
            if j == 0 {
                arr[i][0] = arr[i - 1][j + 1];
            } else if j == 9 {
                arr[i][9] += arr[i - 1][j - 1];
            } else {
                arr[i][j] = arr[i - 1][j - 1] + arr[i - 1][j + 1];
            }

            arr[i][j] %= DIV;
        }
    }

    for i in 0..10 {
        res += arr[n][i];
        res %= DIV;
    }

    println!("{}", res);
}
