use std::io::stdin;

// BOJ 14606

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    // let temp: Vec<u128> = line
    //     .split_whitespace()
    //     .map(|num| num.parse::<u128>().unwrap())
    //     .collect();
    let n = line.trim().parse::<usize>().unwrap();

    let mut enjoy:usize = solve(n);

    println!("{}", enjoy);
}

fn solve(n:usize) -> usize {
    if n == 1 {
        return 0;
    }
    else if n == 2 {
        return 1;
    }
    (n - 1) + solve(n - 1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let mut enjoy:usize = solve(1);
        assert_eq!(0, enjoy);
    }
    #[test]
    fn case2() {
        let mut enjoy:usize = solve(3);
        assert_eq!(3, enjoy);
    }
    #[test]
    fn case3() {
        let mut enjoy:usize = solve(5);
        assert_eq!(10, enjoy);
    }
    #[test]
    fn case4() {
        let mut enjoy:usize = solve(8) ;
        assert_eq!(28, enjoy);
    }
}
