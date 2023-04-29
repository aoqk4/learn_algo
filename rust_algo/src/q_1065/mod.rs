use std::io::stdin;

pub fn q_1065() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let n = numbers[0_usize] as isize;

    let res = solve(n);

    println!("{res}");
}

fn solve(num: isize) -> isize {
    let mut cnt: isize = 0;

    if num < 100 {
        return num;
    } else {
        cnt = 99;

        let mut loo: isize = 100;

        loop {
            if loo > num {
                break;
            }
            let hu = loo / 100;
            let te: isize = (loo / 10) % 10;
            let on = loo % 10;

            if hu - te == te - on {
                cnt += 1;
            }

            loo += 1;
        }
    }
    cnt
}
