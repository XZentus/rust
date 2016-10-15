use std::io;

macro_rules! print_err {
    ($($arg:tt)*) => (
        {
            use std::io::Write;
            writeln!(&mut ::std::io::stderr(), $($arg)*).ok();
        }
    )
}

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    //let l = parse_input!(input_line, i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let h = parse_input!(input_line, i32);

    let mut surface: Vec<Vec<char>> = Vec::with_capacity(h as usize);

    for _ in 0..h as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let row = input_line.trim_right().chars().collect();
        surface.push(row);
    }

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    for _ in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let x = parse_input!(inputs[0], i32);
        let y = parse_input!(inputs[1], i32);
        println!("{}", calc_surf(x, y, &mut surface));
        rewind_surface(&mut surface);
    }
}

fn rewind_surface(surface: &mut Vec<Vec<char>>) {
    for line in surface {
        for point in line {
            if *point == 'X' {
                *point = 'O';
            }
        }
    }
}

fn calc_surf(x: i32, y: i32, surface: &mut Vec<Vec<char>>) -> i32 {
    if x < 0 || y < 0 || y >= surface.len() as i32 || x >= surface[y as usize].len() as i32 || surface[y as usize][x as usize] != 'O' {
        return 0;
    }
    surface[y as usize][x as usize] = 'X';
    let mut ret = 1;
    ret += calc_surf(x - 1, y, surface);
    ret += calc_surf(x + 1, y, surface);
    ret += calc_surf(x, y - 1, surface);
    ret += calc_surf(x, y + 1, surface);
    ret
}
