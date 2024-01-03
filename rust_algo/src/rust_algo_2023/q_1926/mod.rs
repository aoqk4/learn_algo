use std::fmt::Write;
use std::io::stdin;

fn q_1926() {
    let mut output = String::new();

    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let mut visit_arr: [[bool; 501]; 501] = [[false; 501]; 501];

    let mut target_vec: Vec<Vec<usize>> = vec![];

    let n = numbers[0_usize];
    let m = numbers[1_usize];

    let mut queue_vec: Vec<(usize, usize)> = vec![];

    let mut res_vec: Vec<usize> = vec![];

    for _ in 0..n {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");

        let numbers: Vec<usize> = line
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();
        target_vec.push(numbers);
    }
    for col in 0..n {
        for row in 0..m {
            if target_vec[col][row] == 1 && visit_arr[col][row] == false {
                queue_vec.push((col, row));
                visit_arr[col][row] = true;

                let mut cnt = 0;

                loop {
                    if queue_vec.is_empty() {
                        break;
                    }

                    let temp = queue_vec[0];

                    queue_vec.remove(0);
                    cnt += 1;
                    if temp.0 != 0
                        && target_vec[temp.0 - 1][temp.1] == 1
                        && visit_arr[temp.0 - 1][temp.1] == false
                    {
                        queue_vec.push((temp.0 - 1, temp.1));
                        visit_arr[temp.0 - 1][temp.1] = true;
                    }
                    if temp.0 + 1 < n
                        && target_vec[temp.0 + 1][temp.1] == 1
                        && visit_arr[temp.0 + 1][temp.1] == false
                    {
                        queue_vec.push((temp.0 + 1, temp.1));
                        visit_arr[temp.0 + 1][temp.1] = true;
                    }
                    if temp.1 != 0
                        && target_vec[temp.0][temp.1 - 1] == 1
                        && visit_arr[temp.0][temp.1 - 1] == false
                    {
                        queue_vec.push((temp.0, temp.1 - 1));
                        visit_arr[temp.0][temp.1 - 1] = true;
                    }
                    if temp.1 + 1 < m
                        && target_vec[temp.0][temp.1 + 1] == 1
                        && visit_arr[temp.0][temp.1 + 1] == false
                    {
                        queue_vec.push((temp.0, temp.1 + 1));
                        visit_arr[temp.0][temp.1 + 1] = true;
                    }
                }

                res_vec.push(cnt);
                cnt = 0;
            }
        }
    }

    res_vec.sort_by(|a, b| b.cmp(a));

    if res_vec.len() == 0 {
        writeln!(output, "0").unwrap();
        writeln!(output, "0").unwrap();
    } else {
        writeln!(output, "{}", res_vec.len()).unwrap();
        writeln!(output, "{}", res_vec[0]).unwrap();
    }
    print!("{}", output);
}