// use std::fmt::Write;
use std::io::stdin;

// let mut output = String::new();
// wirteln!();

// let mut line = String::new();
// stdin().read_line(&mut line).expect("wrong io");

// let t = line.trim().parse::<usize>().unwrap();

// let numbers: Vec<usize> = line
// .split_whitespace()
// .map(|num| num.parse::<usize>().unwrap())
// .collect();

fn main() {
    let mut sta_vec: Vec<u8> = vec![];

    loop {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");
        if line.trim().to_string() == '.'.to_string() && line.len() == 2 {
            break;
        }
        let mut flag = true;

        for (_, &item) in line.trim().as_bytes().iter().enumerate() {
            if item == b'(' || item == b'[' {
                sta_vec.push(item);
            }
            if sta_vec.len() != 0 {
                if item == b')' && sta_vec[sta_vec.len() - 1] != b'(' {
                    flag = false;
                    break;
                }
                if item == b']' && sta_vec[sta_vec.len() - 1] != b'[' {
                    flag = false;
                    break;
                }
                if item == b')' || item == b']' {
                    sta_vec.pop();
                }
            } else if sta_vec.len() == 0 {
                if item == b')' || item == b']' {
                    flag = false;
                    break;
                }
            }
        }
        if sta_vec.len() != 0 {
            flag = false;
        }
        if flag {
            println!("yes");
        } else {
            println!("no");
        }

        sta_vec.clear();
    }
}

fn sovle() {}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn case1() {}
//     #[test]
//     fn case2() {}
// }
