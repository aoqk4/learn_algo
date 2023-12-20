use std::cell::Cell;
use std::collections::VecDeque;
use std::io::stdin;

fn q_11724() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
    .split_whitespace()
    .map(|num| num.parse::<usize>().unwrap())
    .collect();

    // 방문 했는지?
    let mut vis_arr:[bool; 1200] = [false; 1200];

    // 노드 당 뭐랑 엮여 있는지?(양방향이라서 2차원)
    let mut target_vec:Vec<Vec<Cell<usize>>> = vec![];

    let (n, m) = (numbers[0_usize], numbers[1_usize]);

    // 초기화
    for _ in 0..1200 {
        let row:Vec<Cell<usize>> = vec![];

        target_vec.push(row);
    }


    for _ in 0..m {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");

        let target: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

        let (u, v) = (target[0_usize], target[1_usize]);

        // 각 노드 별 양 방향으로 연결 해준다.
        target_vec[u].push(v.into());
        target_vec[v].push(u.into());
    }

    // 연결 된 수 체크 
    let mut num:usize = 0;

    // 1..n 까지 정점 만큼 반복
    for idx in 1..n + 1 {
        // 방문 했으면?
        if vis_arr[idx] {
            continue;
        }
        
        // 아직 방문 안했으면 -> 연결 요소 있음
        num += 1;

        // 각 노드 별로 BFS 돌리기 위한 준비
        let mut deq_vec:VecDeque<Cell<usize>> = VecDeque::new();
        deq_vec.push_back(idx.into());

        vis_arr[idx] = true;

        // BFS 반복
        loop {
            if deq_vec.is_empty() {
                break;
            }
            let temp = deq_vec[0].get();
            deq_vec.pop_front();

            // 추가로 연결된 노드가 있으면?
            for item in target_vec[temp].iter_mut() {
                // 근데 이미 방문 했으면?
                if vis_arr[item.get()]{
                    continue;
                }

                // 방문 안했으면 -> 추가적으로 연결 요소가 있다는 것
                deq_vec.push_back(item.get().into());
                vis_arr[item.get()] = true;
            }
        }
    }

    println!("{}", num);
}