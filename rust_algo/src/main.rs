use std::fmt::Write;
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

fn sovle() {
}

//
// ***********  알고리즘 포인트들.... ***********
//

// 1. VecDeque
// BFS에서 일반 Vec을 쓰면 remove(idx) call시 인덱스를 한칸 옮겨줘야 해서
// O(N)이 나온다..
// VecDeque를 사용해서 queue나 stack에 이용하자
// 그럼 O(1)로 원소 제거 가능

// 2. BackTracking
// idx를 따지는게 아니라 순서를 따지는 거다
// 일종의 스택을 이용한 DFS임으로(재귀함수의 특징)
// 뻣어나가게 두어서 for문을 이용하면 결국 인덱스와 cnt의 차이가 나는 것.
// 결론은 특정 숫자를 사용했어? 가 아니라 특정 번째 인덱스를 사용했어? 가 되는 것이다.

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn case1() {}
//     #[test]
//     fn case2() {}
// }
