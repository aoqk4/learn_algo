use std::io::stdin;

pub fn q_10773() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let k = numbers[0_usize];

    let mut res_chart: Vec<usize> = vec![];

    let mut total = 0;

    for _ in 0..k {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");

        let numbers: Vec<usize> = line
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        let stk = numbers[0_usize];

        match stk {
            0 => {
                res_chart.pop();
            }
            _ => {
                res_chart.push(stk);
            }
        }
    }

    for i in res_chart.iter() {
        total += i;
    }

    println!("{total}");
}
