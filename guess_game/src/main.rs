extern crate rand;

use std::io::stdin;
use std::io::Write;
use std::io::stdout;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let snum = rand::thread_rng().gen_range(1, 101);
    println!("guess_game>> rnd: {}", snum);

    loop {
        let mut guess = String::new();

        print!("guess_game>> try: ");
        match stdout().flush() {
            Ok(_)  => (),
            Err(_) => (),
        };

        stdin().read_line(&mut guess)
               .ok()
               .expect("Fail read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        match guess.cmp(&snum) {
            Ordering::Less    => println!("guess_game>> Less"),
            Ordering::Equal   => {
                println!("guess_game>> Equal");
                break;
            }
            Ordering::Greater => println!("guess_game>> Greater"),
        }
    }
}
