/// BOJ 2407 https://www.acmicpc.net/problem/2407
// import std input mod
use std::io::stdin;

pub fn q_1407() {
    // 입력을 받는다.
    // READ LINE
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    // LINE SPLIT
    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    // 기준이 되는 숫자
    let n = numbers[0_usize];

    // 현재 센 숫자
    let mut flag: usize = 1;

    // 숫자 올라갈 것
    let mut upper_count = 1;

    // 결과값
    let mut res = 0;

    let mut cnt: usize = 10;

    loop {
        if n < flag {
            break;
        } else {
            if flag < cnt {
                res += upper_count;
            } else {
                upper_count += 1;
                res += upper_count;
                cnt = cnt * 10;
            }
            flag += 1;
        }
    }

    println!("{:?}", res);
}
