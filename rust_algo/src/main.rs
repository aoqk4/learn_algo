// use std::fmt::Write;
// use std::collections::VecDeque;
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

}

// fn sovle() {}


// 
// ***********  알고리즘 포인트들.... ***********
//

// 1. VecDeque
// BFS에서 일반 Vec을 쓰면 remove(idx) call시 인덱스를 한칸 옮겨줘야 해서
// O(N)이 나온다..
// VecDeque를 사용해서 queue나 stack에 이용하자 
// 그럼 O(1)로 원소 제거 가능

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn case1() {}
//     #[test]
//     fn case2() {}
// }
