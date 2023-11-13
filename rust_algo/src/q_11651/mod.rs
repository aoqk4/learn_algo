use std::fmt::Write;
use std::io::stdin;

fn q_11651() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let n = line.trim().parse::<usize>().unwrap();

    let mut target_vec: Vec<(isize, isize)> = vec![];

    let mut output = String::new();

    for _ in 0..n {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");

        let numbers: Vec<isize> = line
            .split_whitespace()
            .map(|num| num.parse::<isize>().unwrap())
            .collect();

        target_vec.push((numbers[0], numbers[1]));
    }

    target_vec.sort_by(|a, b| {
        if a.1 == b.1 {
            return a.0.cmp(&b.0);
        } else {
            return a.1.cmp(&b.1);
        }
    });

    for item in target_vec {
        writeln!(output, "{} {}", item.0, item.1).unwrap();
    }

    print!("{}", output);
}