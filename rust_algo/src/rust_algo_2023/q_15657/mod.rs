fn q_15657() {
    let mut temp_arr = [0; 8];

    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let limit:usize = numbers[1_usize];

    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let mut target_vec: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let mut outp = String::new();

    target_vec.sort();

    sovle(&target_vec, &mut temp_arr, 0, 0, limit, &mut outp);

    print!("{}", outp);
}

fn sovle(target_vec:&[usize], temp_arr:&mut [usize], cnt:usize, idx:usize, limit:usize, outp:&mut String) {
    if cnt == limit {

        for item in 0..limit {
            write!(outp, "{} ", temp_arr[item]).unwrap();
        }
        writeln!(outp, "").unwrap();
        return;
    }
    for item in idx..target_vec.len() {
        temp_arr[cnt] = target_vec[item];
        sovle(target_vec, temp_arr, cnt + 1, item, limit, outp);
    }
    
}