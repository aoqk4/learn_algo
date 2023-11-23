use std::fmt::Write;
use std::io::stdin;

fn q_15656() {
    let mut temp_arr = [0; 8];

    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let limit: usize = numbers[1_usize];

    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let mut target: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    target.sort();

    let mut outp = String::new();

    sovle(&target, &mut temp_arr, 0, limit, &mut outp);

    print!("{}", outp);
}

fn sovle(target: &[usize], temp_arr: &mut [usize], cnt: usize, limit: usize, outp: &mut String) {
    if cnt == limit {
        for item in 0..limit {
            write!(outp, "{} ", temp_arr[item]).unwrap();
        }
        write!(outp, "\n").unwrap();
        return;
    }
    for item in 0..target.len() {
        temp_arr[cnt] = target[item];
        sovle(target, temp_arr, cnt + 1, limit, outp);
    }
}