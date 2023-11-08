use std::io::stdin;
use std::cmp::max;


pub fn q_2559() {    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let temp: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let n = temp[0_usize];
    let k = temp[1_usize];

    // let n = line.trim().parse::<usize>().unwrap();

    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let days: Vec<isize> = line
        .split_whitespace()
        .map(|num| num.parse::<isize>().unwrap())
        .collect();

    let mut start_idx = 0;

    println!("{:?}", solve(&mut start_idx, &days, k));
}

fn solve(start_idx:&mut usize, days:&Vec<isize>, k:usize) -> isize {

    let mut max_val:isize = 0;

    let mut end_idx = *start_idx + k;

    for i in *start_idx..end_idx {
        max_val += days[i];
    }
    *start_idx += 1;
    end_idx = *start_idx + k;

    loop {
        if days.len() < end_idx {
            break
        }
        else {
            let mut temp:isize = 0;

            for i in *start_idx..end_idx {
                temp += days[i];
            }

            max_val = max(max_val, temp);

            temp = 0;
        }
        *start_idx += 1;
        end_idx = *start_idx + k;
    }
    max_val
}