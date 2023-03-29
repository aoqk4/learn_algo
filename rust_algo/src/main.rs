use std::{
    collections::{HashMap, HashSet},
    io::stdin,
};
fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let n: usize = numbers[0_usize];
    let mut sta: Vec<isize> = vec![];
    let mut total: isize = 0;

    for _ in 0..n {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");

        let numbers: Vec<isize> = line
            .split_whitespace()
            .map(|num| num.parse::<isize>().unwrap())
            .collect();

        let input: isize = numbers[0_usize];

        sta.push(input);
        total += input;
    }

    // 산술평균 출력

    let le: isize = sta.len().try_into().unwrap();

    let mut res: f64 = (total as f64 / le as f64).round();

    if res == -0.0 {
        res = 0.0;
    }

    println!("{res}");

    // 중앙값
    let mut sort_sta = sta.clone();
    sort_sta.sort();
    println!("{:?}", sort_sta[n / 2]);

    // 최빈값
    let mut counts: HashMap<isize, isize> = HashMap::new();
    for &num in sta.iter() {
        *counts.entry(num).or_insert(0) += 1;
    }

    let mut mode_vec: Vec<isize> = sta.iter().map(|&num| counts[&num]).collect();

    check_mode(&mut sta, &mut mode_vec);

    // 범위
    let min_val = sta.iter().min().unwrap();
    let max_val = sta.iter().max().unwrap();
    let ran: isize = max_val - min_val;

    println!("{ran}");
}

fn check_mode(arr: &mut Vec<isize>, mode_arr: &mut Vec<isize>) {
    let most: &isize = mode_arr.iter().max().unwrap();
    let mut idx: Vec<usize> = vec![];

    for i in 0..mode_arr.len() {
        if mode_arr[i] == *most {
            idx.push(i);
        }
    }
    if idx.len() == 1 {
        println!("{}", arr[idx[0]]);
        return;
    } else if idx.len() >= 2 {
        let mut res: Vec<isize> = Vec::with_capacity(idx.len());
        for i in idx {
            res.push(arr[i]);
        }
        let res: HashSet<isize> = res.into_iter().collect();
        let mut res: Vec<isize> = res.into_iter().collect();
        res.sort();

        if res.len() == 1 {
            println!("{:?}", res[0]);
            return;
        } else {
            println!("{:?}", res[1]);
        }
    }
}
