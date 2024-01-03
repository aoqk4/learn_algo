use std::fmt::Write;
// use std::collections::VecDeque;
use std::io::stdin;

fn q_15655() {
    let mut numbers = [false; 8];
    let mut temp_arr: [usize; 8] = [0; 8];

    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let inputs: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let m = inputs[1_usize];

    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let mut target_vec: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    target_vec.sort();

    sovle(&mut target_vec, &mut temp_arr, &mut numbers, 0 , 0, m);
}

fn sovle(
    target_vec: &mut Vec<usize>,
    temp_arr: &mut [usize; 8],
    numbers: &mut [bool; 8],
    cnt: usize,
    idx: usize,
    limit: usize,
) {
    if cnt == limit {
        let mut output = String::new();
        for item in 0..limit {
            if item != limit - 1 {
                write!(output, "{} ", temp_arr[item]).unwrap();
            }
            else {
                write!(output, "{}\n", temp_arr[item]).unwrap();
            }
        }
        print!("{}", output);
        return;
    }
    for item in idx..target_vec.len() {
        if !numbers[item] {

            numbers[item] = true;
            temp_arr[cnt] = target_vec[item];
            
            sovle(target_vec, temp_arr, numbers, cnt + 1, item + 1, limit);

            numbers[item] = false;
        }
        
    }
    
}
