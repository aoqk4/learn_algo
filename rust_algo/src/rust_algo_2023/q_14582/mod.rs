// BOJ 14582 https://www.acmicpc.net/problem/14582

use std::io::stdin;

fn q_14582() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let ulim: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let star_link: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let mut ulim_sum:usize = 0;

    let mut star_link_sum:usize = 0;

    if solve(&ulim, &star_link, &mut ulim_sum, &mut star_link_sum) {
        println!("Yes");
    }
    else {
        println!("No");
    }
}

pub fn solve(arr1:&Vec<usize>, arr2:&Vec<usize>, arr1_sum:&mut usize, arr2_sum:&mut usize) -> bool {
    for i in 0..arr1.len() {
        *arr1_sum += arr1[i];
        if arr1_sum > arr2_sum {
            return true;
        }
        *arr2_sum += arr2[i];
    }
    false
}