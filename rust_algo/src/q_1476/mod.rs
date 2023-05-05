use std::io::stdin;

pub fn q_1476() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<isize> = line
        .split_whitespace()
        .map(|num| num.parse::<isize>().unwrap())
        .collect();

    let e = numbers[0_usize];
    let s = numbers[1_usize];
    let m = numbers[2_usize];

    let mut y: isize = 1;

    loop {
        if (y - e) % 15 == 0 && (y - s) % 28 == 0 && (y - m) % 19 == 0 {
            println!("{y}");
            break;
        } else {
            y += 1;
        }
    }
}
