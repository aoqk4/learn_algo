/// BOJ 1459 https://www.acmicpc.net/problem/1459
use std::io::stdin;

fn q_1459() {
    // 입력을 받는다.
    // READ LINE
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    // LINE SPLIT
    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    // x축
    let x = numbers[0_usize];

    // y축
    let y = numbers[1_usize];

    // 한블록 가는 시간
    let w = numbers[2_usize];

    // 가로지르면 걸리는 시간
    let s = numbers[3_usize];

    let case1 = (x + y) * w;

    let mut case2:usize = x.min(y) * s;

    if x > y {
        let mut temp = (x - y) * w;
        if (x - y) % 2 == 1 {
            temp = temp.min(((x - y) - 1) * s + w);
            case2 += temp;
        }
        else {
            temp = temp.min((x - y) * s);
            case2 += temp;
        }
    }
    else {
        let mut temp = (y - x) * w;
        if (y - x) % 2 == 1 {
            temp = temp.min(((y - x) - 1) * s + w);
            case2 += temp;
        }
        else {
            temp = temp.min((y - x) * s);
            case2 += temp;
        }
    }

    println!("{:?}", case1.min(case2));
}