use std::io::stdin;

pub fn q_7568() {
    let mut line = String::new();

    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let n: usize = numbers[0_usize];

    let mut weight: Vec<(usize, usize)> = vec![];
    let mut rank: usize = 1;

    for _ in 0..n {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");

        let numbers: Vec<usize> = line
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        weight.push((numbers[0_usize], numbers[1_usize]));
    }

    for i in 0..n {
        for j in 0..n {
            if weight[i].0 < weight[j].0 && weight[i].1 < weight[j].1 {
                rank += 1;
            }
        }
        print!("{rank} ");
        rank = 1;
    }
    println!();
}
