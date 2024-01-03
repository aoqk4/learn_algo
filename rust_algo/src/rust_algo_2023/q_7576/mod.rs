use std::io::stdin;
use std::collections::VecDeque;

pub fn q_7576() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
    .split_whitespace()
    .map(|num| num.parse::<usize>().unwrap())
    .collect();

    let (n, m) = (numbers[0_usize], numbers[1_usize]);

    let mut target_vec:Vec<Vec<isize>> = vec![];

    // 일반 vec을 사용하면 안되는 이유 -> 일반
    let mut que_vec:VecDeque<(isize, isize)> = VecDeque::new();

    let mov_x = [0, 1, 0, -1];
    let mov_y = [1, 0, -1, 0];

    for _ in 0.. m {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");
    
        let numbers: Vec<isize> = line
        .split_whitespace()
        .map(|num| num.parse::<isize>().unwrap())
        .collect();

        target_vec.push(numbers);
    }

    for col in 0..m {
        for row in 0..n {
            if target_vec[col][row] == 1 {
                que_vec.push_back((col as isize, row as isize));
            }
        }
    }

    loop {
        if que_vec.is_empty() {
            break;
        }
        let temp = que_vec[0];
        que_vec.pop_front();

        for idx in 0..4 {
            let temp_x = temp.1 + mov_x[idx];
            let temp_y = temp.0 + mov_y[idx];

            if temp_x < 0 || temp_x >= n as isize || temp_y < 0 || temp_y >= m as isize {
                continue;
            }
            if target_vec[temp_y as usize][temp_x as usize] == -1 || target_vec[temp_y as usize][temp_x as usize] > 0 {
                continue;
            }

            target_vec[temp_y as usize][temp_x as usize] = target_vec[temp.0 as usize][temp.1 as usize] + 1;
            que_vec.push_back((temp_y, temp_x));
        }
    }

    let mut mx_day:isize = 0;

    for col in 0..m {
        for row in 0..n {
            if target_vec[col][row] == 0 {
                mx_day = -1;
                println!("{}", mx_day);
                return;
            }
            mx_day = mx_day.max(target_vec[col][row] - 1);
        }
    }

    println!("{}", mx_day);
}