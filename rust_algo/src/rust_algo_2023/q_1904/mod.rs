use std::io::stdin;

pub fn q_1904() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();

    let mut arr: Vec<usize> = vec![];

    arr.push(0);
    arr.push(1);
    arr.push(2);

    for i in 3..n + 1 {
        let mut _res: usize = 0;

        _res = arr[i - 1] + arr[i - 2];

        arr.push(_res % 15746);
    }

    println!("{}", arr[n] % 15746);
}
