use std::io::stdin;
use std::fmt::Write;

fn q_2609() {
        // let mut output = String::new();

        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");
    
        // let t = line.trim().parse::<usize>().unwrap();
        
        let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();
    
        let a = numbers[0_usize];
        let b = numbers[1_usize];
    
        println!("{}",gcd(a, b));
        println!("{}",lcm(a, b));
    
        // writeln!();
}


fn lcm(a:usize, b:usize) -> usize {
    (a * b) / gcd(a, b)
}

fn gcd(mut a:usize, mut b:usize) -> usize {
    let mut r:usize = 0;

    loop {
        if b == 0 {
            break;
        }
        else {
            r = a % b;
            a = b;
            b = r;
        }
    }

    a
}