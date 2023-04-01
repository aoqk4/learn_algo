use std::io::stdin;

pub fn q_10828() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let n = numbers[0_usize] as isize;

    let mut stk: Vec<usize> = vec![];

    for _ in 0..n {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");

        let res: Vec<&str> = line.trim().split(' ').collect();

        match res[0] {
            "push" => stk.push(res[1].to_string().parse::<usize>().unwrap()),
            "pop" => {
                if stk.is_empty() {
                    println!("{}", -1);
                } else {
                    let po: usize = stk[stk.len() - 1];
                    println!("{po}");
                    stk.pop();
                }
            }
            "size" => println!("{}", stk.len()),
            "empty" => {
                if stk.is_empty() {
                    println!("{}", 1);
                } else if !stk.is_empty() {
                    println!("{}", 0);
                }
            }
            "top" => {
                if stk.is_empty() {
                    println!("{}", -1);
                } else {
                    println!("{}", stk[stk.len() - 1]);
                }
            }

            _ => todo!(),
        }
    }
}
