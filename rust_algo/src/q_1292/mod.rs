use std::io::stdin;

pub fn q_1292() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let a = numbers[0_usize];
    let b = numbers[1_usize];

    let mut res: usize = 0;
    let mut cnt: usize = 1;

    for i in 1..b + 1 {
        for _ in 0..i {
            if cnt >= a && b >= cnt {
                res += i;
            }
            cnt += 1;
        }
    }
    println!("{res}");
}
