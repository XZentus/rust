use std::io::{stdin};

fn main() {
    let mut inp = String::new();
    stdin().read_line(&mut inp).unwrap();
    let n = inp.trim().parse().unwrap();
    let mut t = vec![false; n];
    
    inp.clear();
    stdin().read_line(&mut inp).unwrap();
    for x in inp.split_whitespace() {
        let   parsed = x.trim().parse::<i32>().unwrap();
        let u_parsed = parsed as usize;
        if parsed > 0 && u_parsed <= n {
            t[u_parsed - 1] = true;
        }
    }
    let result = t.iter().all(|&x| x);

    println!("{}", if result { "YES" }
                   else      { "NO"  });
}
