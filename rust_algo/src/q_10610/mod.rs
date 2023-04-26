use std::io::stdin;

pub fn q_10610() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("wrong io");

    let s_slice: &str = &line[..];

    let mut chars: Vec<char> = s_slice.chars().collect();

    chars.sort_by(|a, b| b.cmp(a));

    let s = String::from_iter(chars);

    let spp: Vec<usize> = s
        .trim()
        .split("")
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();

    if spp[spp.len() - 1] != 0 {
        println!("-1");
    } else {
        let mut div: usize = 0;

        for i in spp.iter() {
            div += i;
        }

        if div % 3 == 0 {
            println!("{s}");
        } else {
            println!("-1");
        }
    }
}
