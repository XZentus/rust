use std::io;

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let v: Vec<usize> = input_line.split_whitespace().map(|x| x.trim().parse::<usize>().unwrap()).collect();
    let n = v[0];
    let k = v[1];

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let mut v: Vec<usize> = input_line.split_whitespace().map(|x| x.trim().parse::<usize>().unwrap()).collect();
    v.sort();
    let mut ans: usize = 0;
    for i in 0..n {
        'inner_loop: for j in i..n {
            if v[i] + k == v[j] { ans += 1; }
            else if v[i] + k < v[j] { break 'inner_loop; };
        }
    }
    println!("{}", ans);
}
