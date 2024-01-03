use std::io::stdin;

pub fn q_11666() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let n: usize = numbers[0_usize];
    let k: usize = numbers[1_usize];

    let mut que: Vec<usize> = vec![];

    for i in 1..n + 1 {
        que.push(i);
    }

    print!("<");

    loop {
        if que.is_empty() {
            break;
        } else {
            for i in 1..k {
                que.push(que[0]);
                que.remove(0);
            }
            print!("{:?}", que[0]);

            if que.len() != 1 {
                print!(", ");
            }
            que.remove(0);
        }
    }

    println!(">");
}
