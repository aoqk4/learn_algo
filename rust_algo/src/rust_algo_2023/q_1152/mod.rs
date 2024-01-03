use std::io::stdin;

fn q_1152() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    println!("{}",sovle(line));
}

fn sovle(line: String) -> usize {
    let sentence: Vec<&str> = line.split_whitespace().collect();
    
    sentence.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(sovle("The Curious Case of Benjamin Button".to_string()), 6);
    }

    #[test]
    fn case2() {
        assert_eq!(sovle("The first character is a blank".to_string()), 6);
    }

    #[test]
    fn case3() {
        assert_eq!(sovle("The last character is a blank ".to_string()), 6);
    }
}