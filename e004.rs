#[inline]
fn is_pal_10(n: u64) -> bool {
    is_pal(n, 10)
}

fn is_pal(n: u64, base: u64) -> bool {
    let mut n2 = n;
    let mut reversed = 0;
    while n2 > 0 {
        reversed = reversed * base + n2 % base;
        n2 /= base;
    }

    reversed == n
}

fn main() {
    let mut largest = 0;
    for a in (100..1000).rev() {
        for b in (a..1000).rev() {
            let c = a * b;
            if c < largest {
                continue;
            }
            else if is_pal_10(c) {
                largest = c;
                // println!("{} * {} = {}", a, b, c);
            }
        }
    }
    println!("\nLargest palindrome = {}", largest);
}
