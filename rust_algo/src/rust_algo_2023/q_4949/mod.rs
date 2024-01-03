use std::io::stdin;

fn q_4949() {
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