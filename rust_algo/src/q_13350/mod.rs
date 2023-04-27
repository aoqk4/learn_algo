use std::io::stdin;

pub fn q_13350() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let n = numbers[0_usize];

    let mut res: usize = 0;
    let mut n_tot: usize = 0;

    let mut costs: Vec<usize> = vec![];
    let mut dists: Vec<usize> = vec![];

    // dists
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    dists = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    // costs
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    costs = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    n_tot = costs[0];

    res += dists[0] * costs[0];

    for i in 1..costs.len() - 1 {
        if costs[i] > n_tot {
            res += n_tot * dists[i];
        } else {
            n_tot = costs[i];
            res += n_tot * dists[i];
        }
    }
    println!("{res}");
}
