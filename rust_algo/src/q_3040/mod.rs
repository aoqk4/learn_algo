use std::io::stdin;

fn q_3040() {
    let mut target_arr: Vec<usize> = vec![];

    for i in 0..9 {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");

        let n = line.trim().parse::<usize>().unwrap();

        target_arr.push(n);
    }

    let solved: Vec<usize> = sovle(&target_arr);

    for item in solved {
        println!("{}", item);
    }
}

fn sovle(target_arr: &Vec<usize>) -> Vec<usize> {
    // 7번을 더해서 100이 되게 하면 됨 -> 9개 고정이니깐 2개를 안더하면 됨

    let mut cl_target: Vec<usize> = vec![];

    let mut mid_temp: usize = 0;

    let mut remove_fir = 0;
    let mut remove_sec = 1;

    loop {
        if mid_temp == 100 {
            break;
        } else {
            if remove_sec == 9 {
                remove_fir += 1;
                remove_sec = remove_fir + 1;
            }
            mid_temp = 0;
            cl_target.clear();
            for i in 0..9 {
                if i != remove_fir && i != remove_sec {
                    cl_target.push(target_arr[i]);
                    mid_temp += target_arr[i];
                }
            }

            remove_sec += 1;
        }
    }
    cl_target
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn case1() {
        let case1_arr: Vec<usize> = vec![7, 8, 10, 13, 15, 19, 20, 23, 25];

        assert_eq!(sovle(&case1_arr), [7, 8, 10, 13, 19, 20, 23]);
    }

    #[test]
    fn case2() {
        let case1_arr: Vec<usize> = vec![8, 6, 5, 1, 37, 30, 28, 22, 36];

        assert_eq!(sovle(&case1_arr), [8, 6, 5, 1, 30, 28, 22]);
    }
}
