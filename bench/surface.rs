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

fn calc_surf(surf: &mut Vec<Vec<u32>>,
             tasks: &mut Vec<(usize, usize)>,
             x_m: usize,
             y_m: usize,
             marker: u32)
             -> usize {
    let mut ret = 0;
    while !tasks.is_empty() {
        let (x, y) = tasks.pop().expect("Tasks ended");
        if surf[y][x] == 0 || surf[y][x] == marker {
            continue;
        } else {
            ret += 1;
            surf[y][x] = marker;
            if y >= 1 {
                tasks.push((x, y - 1))
            };
            if y + 1 < y_m {
                tasks.push((x, y + 1))
            };
            if x >= 1 {
                tasks.push((x - 1, y))
            };
            if x + 1 < x_m {
                tasks.push((x + 1, y))
            };
        }
    }
    ret
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let l = parse_input!(input_line, usize);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let h = parse_input!(input_line, usize);

    let mut map: Vec<Vec<u32>> = Vec::with_capacity(h);
    for _ in 0..h {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let row: Vec<u32> = input_line.trim_right()
            .bytes()
            .map(|x| if x == b'#' {
                0
            } else {
                1
            })
            .collect();
        map.push(row);
    }
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, usize);
    let mut marker = 1;

    let mut tasks: Vec<(usize, usize)> = Vec::new();
    for _ in 0..n {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.trim_right().split(" ").collect::<Vec<_>>();
        tasks.push((inputs[0].parse().unwrap(), inputs[1].parse().unwrap()));
        marker += 1;
        println!("{}", calc_surf(&mut map, &mut tasks, l, h, marker));
    }
}
