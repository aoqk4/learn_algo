use std::io::stdin;

pub fn q_14606() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let n = line.trim().parse::<usize>().unwrap();

    let mut enjoy:usize = solve(n);

    println!("{}", enjoy);
}
fn solve(n:usize) -> usize {
    if n == 1 {
        return 0;
    }
    else if n == 2 {
        return 1;
    }
    (n - 1) + solve(n - 1)
}