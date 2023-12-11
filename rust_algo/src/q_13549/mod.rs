use std::collections::VecDeque;
use std::io::stdin;

fn q_13549() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let mut target_arr: [usize; 100001] = [0; 100001];
    let mut vis_arr: [bool; 100001] = [false; 100001];

    let numbers: Vec<isize> = line
    .split_whitespace()
    .map(|num| num.parse::<isize>().unwrap())
    .collect();

    let (n, m) = (numbers[0_usize], numbers[1_usize]);

    let mut que_vec: VecDeque<isize> = VecDeque::new();

    que_vec.push_back(n);

    loop {
        let temp = que_vec[0];
        que_vec.pop_front();
        if temp == m{
            break;
        }

        if temp + 1 == m || temp - 1 == m {
            target_arr[m as usize] = target_arr[temp as usize] + 1;
            break;
        }
        if temp * 2 <= m + 1 as isize {
            vis_arr[(temp * 2) as usize] = true;
            target_arr[(temp * 2) as usize] = target_arr[temp as usize];
            que_vec.push_back(temp * 2);
        }
        if temp > 0 && !vis_arr[(temp - 1) as usize] {
            target_arr[(temp - 1) as usize] = target_arr[temp as usize] + 1;
            que_vec.push_back(temp - 1);
        }
        if temp + 1 < target_arr.len() as isize && !vis_arr[(temp + 1) as usize] {
            target_arr[(temp + 1) as usize] = target_arr[temp as usize] + 1;
            que_vec.push_back(temp + 1);
        }
    }

    println!("{}", target_arr[m as usize]);
}