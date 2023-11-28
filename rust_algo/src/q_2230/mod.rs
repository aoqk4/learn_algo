use std::io::stdin;

pub fn q_2230() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<isize> = line
        .split_whitespace()
        .map(|num| num.parse::<isize>().unwrap())
        .collect();

    let (n ,m) = (numbers[0_usize], numbers[1_usize]);

    let mut target_arr:Vec<isize> = vec![];

    for _ in 0..n {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");

        let item = line.trim().parse::<isize>().unwrap();

        target_arr.push(item);
    }

    target_arr.sort();

    let mut min_val: isize = 0x6fffffff;
    let mut start_idx = 0;
    let mut end_idx = 0; 

    loop {
        if start_idx == n || end_idx == n {
            break;
        }
        else if target_arr[end_idx as usize] - target_arr[start_idx as usize] < m {
            end_idx += 1;
        }
        else {
            min_val = min_val.min(target_arr[end_idx as usize] - target_arr[start_idx as usize]);
            start_idx += 1;
        }
    }

    println!("{:?}", min_val);
}