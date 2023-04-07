use std::io::stdin;
pub fn q_2606() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let n = numbers[0_usize];

    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let k: usize = numbers[0_usize];

    let mut res_v: Vec<(bool, bool)> = vec![(false, false); n + 1];
    let mut res: Vec<(usize, usize)> = vec![];
    let mut cnt: usize = 0;

    for _ in 0..k {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");

        let numbers: Vec<usize> = line
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        res.push((numbers[0_usize], numbers[1_usize]));
    }
    solve(&mut res, &mut res_v, &mut cnt, 1);
    println!("{cnt}");
}

fn solve(
    res: &mut Vec<(usize, usize)>,
    res_v: &mut Vec<(bool, bool)>,
    cnt: &mut usize,
    search: usize,
) {
    res_v[1] = (true, true);

    for i in 0..res.len() {
        if res[i].0 == search {
            res_v[search].0 = true;
            if res_v[res[i].1] == (false, false) {
                res_v[res[i].1] = (true, true);
                *cnt += 1;
                solve(res, res_v, cnt, res[i].1);
            }
        }
        if res[i].1 == search {
            res_v[search].1 = true;
            if res_v[res[i].0] == (false, false) {
                res_v[res[i].1] = (true, true);
                *cnt += 1;
                solve(res, res_v, cnt, res[i].0);
            }
        }
    }
}
