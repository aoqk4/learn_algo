use std::io::stdin;

pub fn a_2960() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let n = numbers[0_usize];

    let k = numbers[1_usize];

    let mut usize_vec: Vec<usize> = vec![];
    let mut res_vec: Vec<usize> = vec![];

    for i in 2..n + 1 {
        usize_vec.push(i);
    }

    loop {
        if usize_vec.len() == 0 {
            break;
        }
        che(&mut usize_vec, &mut res_vec);
    }

    println!("{:?}", res_vec[k - 1]);
}

fn che(usize_vec: &mut Vec<usize>, res_vec: &mut Vec<usize>) {
    usize_vec.sort();

    let p = usize_vec[0];
    res_vec.push(usize_vec[0]);
    usize_vec.remove(0);

    let mut cnt: usize = 0;

    loop {
        if usize_vec.len() <= cnt {
            break;
        }

        if usize_vec[cnt] % p == 0 {
            res_vec.push(usize_vec[cnt]);
            usize_vec.remove(cnt);
        }
        cnt += 1;
    }
}
