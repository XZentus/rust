use std::io;
use std::cmp::min;

fn main() {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).unwrap();
    let n = inp.trim().parse().unwrap_or(1);
    
    let mut data: Vec<Vec<u64>> = Vec::with_capacity(n);
    for _ in 0..n {
        inp.clear();
        io::stdin().read_line(&mut inp).unwrap();
        //let t = inp.split_whitespace().map(|x| x.trim().parse().unwrap_or(0)).collect();
        let t = inp.split(',').map(|x| x.trim().parse().unwrap_or(0)).collect();
        data.push(t);
    }

    for d in 1..2*n {
        let mut col = d as isize;
        let mut row = 0 as isize;
        let n = n as isize;
        while col >= 0 && row < n {
            if col < n {
                let col = col as usize;
                let row = row as usize;
                data[row][col] += {
                    if      col == 0 { data[row - 1][col] }
                    else if row == 0 { data[row][col - 1] }
                    else { min(data[row - 1][col], data[row][col - 1]) }
                };
            }
            col -= 1;
            row += 1;
        }
    }

    println!("{}", data[n - 1][n - 1]);
}