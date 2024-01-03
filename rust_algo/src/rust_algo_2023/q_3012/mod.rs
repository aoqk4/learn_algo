use std::io::stdin;

fn q_3012() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let n = line.trim().parse::<usize>().unwrap();

    let mut rank_vec: Vec<usize> = vec![];

    for _ in 0..n {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");

        let n = line.trim().parse::<usize>().unwrap();

        rank_vec.push(n);
    }

    rank_vec.sort();

    let sovle_score = solve(&rank_vec);

    println!("{}", sovle_score);
}

fn solve(rank_vec:&Vec<usize>) -> usize {
    let mut min_ang:usize = 0;

    for i in 0..rank_vec.len() {
        min_ang += (i + 1).abs_diff(rank_vec[i]);
    }

    min_ang
}
