use std::io::stdin;

pub fn q_3036() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let n = line.trim().parse::<usize>().unwrap();

    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let mut res: usize = 0;

    let ring: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    for i in 1..n {
        if ring[0] > ring[i] {
            res = solve(ring[0], ring[i]);
        } else {
            res = solve(ring[i], ring[0]);
        }

        println!("{}/{}", ring[0] / res, ring[i] / res);
    }
}

fn solve(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    solve(b, a % b)
}
