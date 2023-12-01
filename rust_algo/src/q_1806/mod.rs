fn q_1806() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let (n, s) = (numbers[0_usize], numbers[1_usize]);

    let mut end_idx = 0;

    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let mut mid_val = numbers[0];
    let mut mid_length = i32::MAX;

    for idx in 0..n {
        while end_idx < n && mid_val < s {
            end_idx += 1;

            if end_idx != n {
                mid_val += numbers[end_idx];
            }
        }
        if end_idx == n {
            break;
        }
        mid_length = mid_length.min((end_idx - idx + 1) as i32);
        mid_val -= numbers[idx];
    }

    if mid_length == i32::MAX {
        mid_length = 0;
    }

    println!("{:?}", mid_length);
}