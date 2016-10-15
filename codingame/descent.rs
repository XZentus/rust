use std::io;

macro_rules! print_err {
    ($($arg:tt)*) => (
        {
            use std::io::Write;
            writeln!(&mut ::std::io::stderr(), $($arg)*);
        }
    )
}

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let space_x = parse_input!(inputs[0], i32);
        // let space_y = parse_input!(inputs[1], i32);
        let mut max_h = 0;
        let mut x_h: i32 = 0;
        for i in 0..8 as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let mountain_h = parse_input!(input_line, i32); // represents the height of one mountain, from 9 to 0. Mountain heights are provided from left to right.
            if max_h < mountain_h {
              max_h = mountain_h;
              x_h = i as i32;
            }
        }
        if space_x == x_h {
          println!("FIRE");
        }
        else {
          println!("HOLD");
        }

        // Write an action using println!("message...");
        // To debug: print_err!("Debug message...");

        // println!("HOLD"); // either:  FIRE (ship is firing its phase cannons) or HOLD (ship is not firing).
    }
}
