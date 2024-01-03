// use std::collections::VecDeque;
use std::collections::BTreeSet;
use std::fmt::Write;
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

// #[cfg(test)]
// mod test {
//     #[test]
//     fn case1() {

//     }
// }

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

// 2. BackTracking
// idx를 따지는게 아니라 순서를 따지는 거다
// 일종의 스택을 이용한 DFS임으로(재귀함수의 특징)
// 뻣어나가게 두어서 for문을 이용하면 결국 인덱스와 cnt의 차이가 나는 것.
// 결론은 특정 숫자를 사용했어? 가 아니라 특정 번째 인덱스를 사용했어? 가 되는 것이다.

// 3. 이분탐색과 BTreeSet
// 다른 경우에
// 이분 탐색할 때는 lower_bound와 upper_bound를 요구할 때도 많고
// 애초에 그냥 vec에서 원소 넣으면 중복값 없애주면서 btree 만들어 주는
// BTreeSet 이용해서 풀도록 하자.

// 4. loop vs while ?
// 종료 조건이 있으면 loop
// 돌아가야 하는 조건이 있으면 while을 사용하는게 좋을 것 같다.

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn case1() {}
//     #[test]
//     fn case2() {}
// }
