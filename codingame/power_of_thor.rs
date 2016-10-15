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
 * ---
 * Hint: You can use the debug stream to print initialTX and initialTY, if Thor seems not follow your orders.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let light_x = parse_input!(inputs[0], i32); // the X position of the light of power
    let light_y = parse_input!(inputs[1], i32); // the Y position of the light of power
    let mut x = parse_input!(inputs[2], i32); // Thor's starting X position
    let mut y = parse_input!(inputs[3], i32); // Thor's starting Y position

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        // let remaining_turns = parse_input!(input_line, i32); // The remaining amount of turns Thor can move. Do not remove this line.

        // Write an action using println!("message...");
        // To debug: print_err!("Debug message...");
        let ux = unit_move(light_x - x);
        let uy = unit_move(light_y - y);
        x += ux;
        y += uy;
        print_move(ux, uy);

//        println!("SE"); // A single line providing the move to be made: N NE E SE S SW W or NW
    }
}

fn unit_move (mv: i32) -> i32 {
  if mv > 0 { return 1}
  else if mv < 0 {return -1}
  else {return 0};
}

fn print_move(dx: i32, dy: i32) {
  match dy {
    1  => print!("S"),
    -1 => print!("N"),
    _  => (),
  };

  match dx {
    1  => print!("E"),
    -1 => print!("W"),
    _  => (),
  };
  println!("");
}
