use std::io::stdin;

fn q_3273() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let _n = line.trim().parse::<usize>().unwrap();

    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let mut numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let x = line.trim().parse::<usize>().unwrap();

    let mut s_idx = 0;
    let mut e_idx = numbers.len() - 1;

    numbers.sort();

    let mut cnt = 0;

    loop {
        if s_idx >= e_idx {
            break;
        }
        if numbers[s_idx] + numbers[e_idx] == x {
            cnt += 1;
            s_idx += 1;
            e_idx -= 1;
        } else if numbers[s_idx] + numbers[e_idx] > x {
            e_idx -= 1;
        } else {
            s_idx += 1;
        }
    }

    println!("{}", cnt);
}