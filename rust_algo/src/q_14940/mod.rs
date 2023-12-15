use std::collections::VecDeque;
use std::io::stdin;
use std::fmt::Write;

fn q_14940() {
    let mut output = String::new();

    let mut target_vec: Vec<Vec<isize>> = vec![];

    let mut target_x = 0;
    let mut target_y = 0;

    let mov_x = [0, 1, 0, -1];
    let mov_y = [1, 0, -1, 0];

    let mut vis_arr:[[bool; 1000]; 1000] = [[false; 1000]; 1000];

    let mut de_vec: VecDeque<(isize, isize)> = VecDeque::new();

    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let (n, m) = (numbers[0_usize], numbers[1_usize]);

    for _ in 0..n {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");

        let target: Vec<isize> = line
            .split_whitespace()
            .map(|num| num.parse::<isize>().unwrap())
            .collect();

        target_vec.push(target);
    }

    for col in 0..n {
        for row in 0..m {
            if target_vec[col][row] == 2 {
                target_x = row as isize;
                target_y = col as isize;

                target_vec[col][row] = 0
            }
        }
    }

    de_vec.push_back((target_x, target_y));

    loop {
        if de_vec.is_empty() {
            break;
        }
        let temp = de_vec[0];
        de_vec.pop_front();

        for idx in 0..4 {
            let temp_x = temp.0 + mov_x[idx];
            let temp_y = temp.1 + mov_y[idx];

            if temp_x < 0 || temp_x >= m as isize || temp_y < 0 || temp_y >= n as isize {
                continue;
            }
            if target_vec[temp_y as usize][temp_x as usize] == 0 || vis_arr[temp_y as usize][temp_x as usize]{
                continue;
            }

            if !vis_arr[temp_y as usize][temp_x as usize] {

                target_vec[temp_y as usize][temp_x as usize] = target_vec[temp.1 as usize][temp.0 as usize] + 1;

                vis_arr[temp_y as usize][temp_x as usize] = true;

                de_vec.push_back((temp_x, temp_y));
            }
            
        }
    }

    for col in 0..n {
        for row in 0..m {
            if vis_arr[col][row] == false && target_vec[col][row] != 0 {
                write!(output, "{} ", -1).unwrap();
            }
            else {
                write!(output, "{} ", target_vec[col][row]).unwrap();
            }
        }
        writeln!(output, "").unwrap();
    }

    print!("{}", output);
}