fn main() {
    let mut n: u64 = 600851475143;

    let mut d = 3;
    while n != d {
        if n % d == 0 {
            n /= d;
        }
        else {
            d += 2;
        }
    }

    println!("{}", d);
}
