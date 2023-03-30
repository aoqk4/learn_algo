use std::io::stdin;

pub fn q_14719() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let h = numbers[0_usize] as isize;
    let w = numbers[1_usize];

    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let mut h_vec: Vec<isize> = line
        .split_whitespace()
        .map(|num| num.parse::<isize>().unwrap())
        .collect();
    let mut res: Vec<Vec<bool>> = vec![];

    let mut in_vec: Vec<bool> = vec![];

    let mut temp = h - 1;

    for i in 0..w {
        loop {
            if temp < 0 {
                break;
            }
            if h_vec[i] > 0 {
                in_vec.push(true);
            } else {
                in_vec.push(false);
            }
            temp -= 1;
            h_vec[i] -= 1;
        }
        in_vec.reverse();
        res.push(in_vec.clone());
        in_vec.clear();
        temp = h - 1;
    }

    let mut record: bool = false;

    let mut r_total: usize = 0;
    let mut total: usize = 0;

    for i in 0..h {
        for j in 0..w {
            if res[j][i as usize] == true && record == false {
                record = true;
                continue;
            } else if res[j][i as usize] == true && record == true {
                total += r_total;
                r_total = 0;
            }
            if record == true && res[j][i as usize] == false {
                r_total += 1;
            }
        }
        r_total = 0;
        record = false;
    }
    println!("{total}");
}
