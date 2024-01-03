use std::io::stdin;
use std::fmt::Write;

pub fn q_9461() {
    let mut output = String::new();

    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let t = line.trim().parse::<usize>().unwrap();

    let mut target_vec:Vec<usize> = vec![1,1,1,2,2,3,4,5,7,9];
    
    for item in 10..100 {
        target_vec.push(target_vec[item - 2] + target_vec[item - 3]);
    }

    for _ in 0..t {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");

        let n = line.trim().parse::<usize>().unwrap();

        writeln!(output, "{}", target_vec[n - 1]).unwrap();
    }

    print!("{}", output);
}