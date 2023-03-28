use std::io::stdin;

pub struct Coun {
    gol: usize,
    sil: usize,
    cou: usize,
}

impl Coun {
    fn new(gol: usize, sil: usize, cou: usize) -> Self {
        Self { gol, sil, cou }
    }
}

pub fn q_8979() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let numbers: Vec<usize> = line
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let n = numbers[0_usize];
    let k = numbers[1_usize];
    let mut arr: Vec<(usize, Coun)> = vec![];

    let mut rank: usize = 1;
    let mut rank_arr: Vec<usize> = vec![];

    for _ in 0..n {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("wrong io");

        let numbers: Vec<usize> = line
            .split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        let country = numbers[0_usize];
        let g = numbers[1_usize];
        let s = numbers[2_usize];
        let c = numbers[3_usize];

        arr.push((country, Coun::new(g, s, c)));
        rank_arr.push(0);
    }

    for i in 0..n {
        for j in 0..n {
            if arr[i].1.gol < arr[j].1.gol {
                rank += 1;
            } else if arr[i].1.gol == arr[j].1.gol && arr[i].1.sil < arr[j].1.sil {
                rank += 1;
            } else if arr[i].1.gol == arr[j].1.gol
                && arr[i].1.sil == arr[j].1.sil
                && arr[i].1.cou < arr[j].1.cou
            {
                rank += 1;
            }
        }
        rank_arr[arr[i].0 - 1] = rank;
        rank = 1;
    }
    println!("{:?}", rank_arr[k - 1]);
}
