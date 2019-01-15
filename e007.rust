fn init_primes(target_capacity: usize) -> Vec<u64> {
    let mut vec = Vec::<u64>::with_capacity(target_capacity);

    for prime in [2, 3, 5, 7, 11, 13, 17].iter() {
        vec.push(*prime);
    }

    let mut next_prime = vec[vec.len() - 1] + 2;
    while vec.len() < target_capacity {
        if is_prime(next_prime, &vec) {
            vec.push(next_prime);
        }
        next_prime += 2;
    }

    vec
}

fn is_prime(n: u64, primes: &Vec<u64>) -> bool {
    for p in primes {
        if n % p == 0 {
            return false;
        }
        if p * p > n {
            break;
        }
    }
    true
}

fn main() {
    let primes = init_primes(10001);

    println!("{}", primes[primes.len() - 1]);
}
