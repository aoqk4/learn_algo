use std::io::stdin;

pub fn q_1402() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let t: usize = numbers[0_usize];

    for _ in 0..t {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");

        let numbers: Vec<isize> = line
            .split_whitespace()
            .map(|num| num.parse::<isize>().unwrap())
            .collect();

        let _a: isize = numbers[0_usize] as isize;
        let _b: isize = numbers[1_usize] as isize;

        println!("yes");
    }
}
