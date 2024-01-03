fn q_13301() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    // let temp: Vec<u128> = line
    //     .split_whitespace()
    //     .map(|num| num.parse::<u128>().unwrap())
    //     .collect();
    let n = line.trim().parse::<u128>().unwrap();

    let mut fivo_arr:Vec<u128> = vec![];

    println!("{:?}", solve(n, &mut fivo_arr));
}
fn solve(n:u128, vec:&mut Vec<u128>) -> u128 {

    if n == 1 {
        return 4;
    }

    let mut temp1:u128 = 0;
    let mut temp2:u128 = 1;
    let mut cnt = 1;
    let mut tot = 0;

    loop {
        if cnt > n {
            vec.push(tot);
            break
        }
        else {
            tot = temp1 + temp2;
            temp1 = temp2;
            temp2 = tot;
        }

        if cnt == n - 1 {
            vec.push(tot);
        }

        cnt += 1;
    }

    let res:u128 = vec.iter().sum::<u128>() * 2;
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let mut case_vec:Vec<u128> = vec![];
        assert_eq!(4, solve(1, &mut case_vec));
    }
    #[test]
    fn case2() {
        let mut case_vec:Vec<u128> = vec![];
        assert_eq!(42, solve(6, &mut case_vec));
    }
}