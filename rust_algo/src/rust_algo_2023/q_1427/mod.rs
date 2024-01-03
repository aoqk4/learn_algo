use std::io::stdin;

pub fn q_1427() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let mut arr: Vec<usize> = line
        .trim()
        .split("")
        .filter_map(|x| x.parse().ok())
        .collect();

    arr.sort();
    arr.reverse();

    for i in arr {
        print!("{}", i);
    }
    println!();
}
