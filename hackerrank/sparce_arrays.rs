use std::io;

fn main() {
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();
    let n: usize = input_line.trim().parse().unwrap();
    let mut strings: Vec<String> = Vec::with_capacity(n);

    for _ in 0..n {
        input_line.clear();
        io::stdin().read_line(&mut input_line).unwrap();
        //input_line.trim();
        strings.push(input_line.clone());
    }

    input_line.clear();
    io::stdin().read_line(&mut input_line).unwrap();
    let q: usize = input_line.trim().parse().unwrap();

    for _ in 0..q {
        input_line.clear();
        io::stdin().read_line(&mut input_line).unwrap();
        println!("{}", count_strs(&input_line, &strings));
    }
}

fn count_strs(querie: &String, strings: &Vec<String>) -> usize {
    strings.iter().fold(0, |sum, quer| {if quer == querie { sum + 1} else { sum }})
}
