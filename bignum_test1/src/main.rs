extern crate num_bigint;
extern crate num_traits;
extern crate num;

use num_bigint::BigUint;
use num::FromPrimitive;
use num_traits::Zero;
use std::env;
use std::mem::replace;

fn eukl(m: &mut u64, n: &mut u64) -> usize {
    println!("------------------");
    println!("Begin for m = {} n = {}", m, n);
    let mut ret = 0;
    let mut r;
    loop {
        r = *m % *n;
        ret += 1;
        if r == 0 { break; }
        *m = *n;
        *n = r;
    }
    println!("Result: {}", ret);
    println!("------------------\n");
    ret
}

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let (mut a1, mut a2): (BigUint, BigUint) =
        if args.len() == 3 { (
            args[1].trim().parse().unwrap_or(FromPrimitive::from_u64(675467876893).unwrap()),
            args[2].trim().parse().unwrap_or(FromPrimitive::from_u64(656478347387).unwrap()) ) }
        else { (
            FromPrimitive::from_u64(56347856323).unwrap(),
            FromPrimitive::from_u64(54365783432).unwrap() ) };

    let mut r: BigUint;
    let mut n_iter = 0;

    loop {
        println!("a1 = {} a2 = {}", a1, a2);
        r = a1.clone() % a2.clone();
        if r == Zero::zero() { break; }
        a1 = replace(&mut a2, r);
        n_iter += 1;
    }
    println!("gcd = {}, iterations = {}", a2, n_iter);

    let max_lim = 100;
    let mut med: f64 = 0f64;
    for x in 0..max_lim + 1 {
        let mut x = x;
        med += eukl(&mut x, &mut 5u64) as f64;
    }
    println!("med = {}", med / max_lim as f64);
}
