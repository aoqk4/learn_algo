fn q_7785() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let mut output = String::new();

    let n = line.trim().parse::<usize>().unwrap();

    let mut name_hash: HashMap<String, String> = HashMap::new();

    let mut res_vec: Vec<String> = vec![];

    for _ in 0..n {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");

        let target: Vec<String> = line.split_whitespace().map(|str| str.to_string()).collect();

        if target[1] == "enter" {
            name_hash.insert(target[0].clone(), "enter".to_string());
        } else {
            name_hash.remove(&target[0]).unwrap();
        }
    }

    for item in name_hash {
        res_vec.push(item.0);
    }

    res_vec.sort_by(|a, b| b.cmp(a));

    for item in res_vec {
        writeln!(output, "{}", item).unwrap();
    }

    print!("{}", output);
}