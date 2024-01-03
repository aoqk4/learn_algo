fn q_10866() {
    let mut output = String::new();

    let mut vec_deq:VecDeque<usize> = VecDeque::new();

    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let n = line.trim().parse::<usize>().unwrap();

    for _ in 0..n {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");

        let target_vec:Vec<&str> =  line.split_whitespace().collect();

        if target_vec.len() == 1 {
            if target_vec[0] == "size" {
                writeln!(output, "{}", vec_deq.len()).unwrap();
            }
            else if target_vec[0] == "empty" {
                if vec_deq.is_empty() {
                    writeln!(output, "{}", 1).unwrap();
                }
                else {
                    writeln!(output, "{}", 0).unwrap();
                }
            }
            else if target_vec[0] == "pop_front" {
                if vec_deq.is_empty() {
                    writeln!(output, "{}", -1).unwrap();
                }
                else {
                    writeln!(output, "{}", vec_deq.front().unwrap()).unwrap();
                    vec_deq.pop_front();
                }
            }
            else if target_vec[0] == "pop_back" {
                if vec_deq.is_empty() {
                    writeln!(output, "{}", -1).unwrap();
                }
                else {
                    writeln!(output, "{}", vec_deq.back().unwrap()).unwrap();
                    vec_deq.pop_back();
                }
            }
            else if target_vec[0] == "front" {
                if vec_deq.is_empty() {
                    writeln!(output, "{}", -1).unwrap();
                }
                else {
                    writeln!(output, "{}", vec_deq.front().unwrap()).unwrap();
                }
            }
            else if target_vec[0] == "back" {
                if vec_deq.is_empty() {
                    writeln!(output, "{}", -1).unwrap();
                }
                else {
                    writeln!(output, "{}", vec_deq.back().unwrap()).unwrap();
                }
            }
        }
        else {
            if target_vec[0] == "push_front"{
                vec_deq.push_front(target_vec[1].parse().unwrap());
            }
            else {
                vec_deq.push_back(target_vec[1].parse().unwrap());
            }
        }
    }

    print!("{}",output);
}