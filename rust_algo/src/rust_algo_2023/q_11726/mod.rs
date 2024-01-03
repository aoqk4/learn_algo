use std::io::stdin;

pub fn q_11726() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let n: usize = numbers[0_usize];

    let mut arr: Vec<usize> = vec![0; 1001];

    arr[1] = 1;
    arr[2] = 2;

    for i in 3..n + 1 {
        arr[i] = arr[i - 1] + arr[i - 2];
        arr[i] %= 10007;
    }

    println!("{}", arr[n]);
}
