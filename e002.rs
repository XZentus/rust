fn main() {
    let mut f1: u64 = 1;
    let mut f2: u64 = 2;

    let mut sum = 0;

    while f2 < 4_000_000 {
        if f2 % 2 == 0 {
            sum += f2;
        }

        let ftemp = f1;
        f1 = f2;
        f2 += ftemp;
    }

    println!("{}", sum);
}
