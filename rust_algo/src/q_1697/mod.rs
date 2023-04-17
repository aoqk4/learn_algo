use std::io::stdin;

pub fn q_1697() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let n1: usize = numbers[0_usize];
    let n2: usize = numbers[1_usize];

    let mut vis: Vec<bool> = vec![false; 100001];

    vis[n1] = true;

    solve(&mut vis, n1.clone(), n2.clone());
}

fn solve(vis: &mut Vec<bool>, mut n1: usize, n2: usize) {
    let mut has: Vec<(usize, usize)> = vec![];
    let mut cnt = 0;
    has.push((n1, 0));
    loop {
        if !has.is_empty() {
            n1 = has[0].0;
            cnt = has[0].1;
            has.remove(0);
        }
        if n1 == n2 {
            println!("{cnt}");
            break;
        }
        if n1 + 1 < vis.len() && !vis[n1 + 1] {
            vis[n1 + 1] = true;
            has.push((n1 + 1, cnt + 1));
        }
        if n1 - 1 < vis.len() && !vis[n1 - 1] {
            vis[n1 - 1] = true;
            has.push((n1 - 1, cnt + 1));
        }
        if n1 * 2 < vis.len() && !vis[n1 * 2] {
            vis[n1 * 2] = true;
            has.push((n1 * 2, cnt + 1));
        }
    }
}
