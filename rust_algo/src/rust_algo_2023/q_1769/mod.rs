use std::io::stdin;

pub fn q_1769() {
    // 입력 받고
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let mut total: u32 = 0;
    let mut cnt: usize = 0;

    loop {
        if line.trim().len() == 1 {
            total = line.chars().next().unwrap().to_digit(10).unwrap();

            println!("{cnt}");

            match total % 3 {
                0 => println!("YES"),
                _ => println!("NO"),
            }
            break;
        }
        total = 0;
        for c in line.trim().chars() {
            total += c.to_digit(10).unwrap();
        }

        line = total.to_string();
        cnt += 1;
    }
}
