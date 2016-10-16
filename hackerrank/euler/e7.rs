use std::io::{stdin};

const LIMIT: usize = 10002;

fn main() {
    let primes = init_primes(LIMIT);
    let mut inp = String::new();
    stdin().read_line(&mut inp).unwrap();
    let t: usize = inp.trim().parse().unwrap();
    
    for _ in 0..t {
        inp.clear();
        stdin().read_line(&mut inp).unwrap();
        let q: usize = inp.trim().parse().unwrap();
        println!("{}", primes[q]);
    }
}

fn init_primes(lim: usize) -> Vec<u64> {
    let mut data: Vec<u64> = Vec::with_capacity(lim + 2);
    data.push(1);
    data.push(2);
    data.push(3);
    for _ in 0..lim - 2 {
        let mut t = *data.last().unwrap();
        'next_t: loop {
            t += 2;
            for j in 1..data.len() - 1 {
                if t % data[j] == 0 {
                    continue 'next_t;
                }
            }
            break;
        }
        data.push(t);
    }
    data
}