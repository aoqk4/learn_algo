use std::io::stdin;

pub fn q_1676() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let n: usize = numbers[0_usize];

    let mut res: usize = 0;

    let mut cnt = 5;

    loop {
        if cnt > n {
            break;
        }

        res += n / cnt;

        cnt *= 5;
    }

    println!("{res}");
}
