use std::io::stdin;

pub fn q_1074() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let n = numbers[0_usize];
    let r = numbers[1_usize];
    let c = numbers[2_usize];

    let mut cnt: usize = 0;

    solve(0, 0, 1 << n, r, c, &mut cnt);
}

fn solve(x: usize, y: usize, size: usize, r: usize, c: usize, cnt: &mut usize) {
    if y == r && x == c {
        println!("{}", cnt);
        return;
    } else if r < y + size && r >= y && c < x + size && c >= x {
        solve(x, y, size / 2, r, c, cnt);
        solve(x + size / 2, y, size / 2, r, c, cnt);
        solve(x, y + size / 2, size / 2, r, c, cnt);
        solve(x + size / 2, y + size / 2, size / 2, r, c, cnt);
    } else {
        *cnt += size * size;
    }
}
