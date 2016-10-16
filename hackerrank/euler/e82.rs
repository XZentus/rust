use std::io;
use std::cmp::min;

fn main() {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).unwrap();
    let n = inp.trim().parse().unwrap();
    
    let mut data: Vec<Vec<(u64, u64)>> = Vec::with_capacity(n);
    for _ in 0..n {
        inp.clear();
        io::stdin().read_line(&mut inp).unwrap();
        let t = inp.split_whitespace().map(|x| (0, x.trim().parse().unwrap())).collect();
        //let t = inp.split(',').map(|x| x.trim().parse().unwrap_or(0)).collect();
        data.push(t);
    }
    println!("{}", calc_matrix(&mut data));
}

fn calc_matrix(data: &mut Vec<Vec<(u64, u64)>>) -> u64 {
    let n = data.len();
    for col in 1..n {
        for row in 0..n {
            data[row][col].0 = data[row][col - 1].0;
        }
        {
            let next_row = data[1][col].0     + data[1][col].1;
            let prev_col = data[0][col - 1].0 + data[0][col - 1].1;
            data[0][col].0 = min(next_row, prev_col);
        }
        for row in 1..n - 1 {
            let next_row = data[row + 1][col].0 + data[row + 1][col].1;
            let prev_row = data[row - 1][col].0 + data[row - 1][col].1;
            let prev_col = data[row][col - 1].0 + data[row][col - 1].1;
            data[row][col].0 = min(min(next_row, prev_row), prev_col);
        }
        {
            let prev_row = data[n - 2][col].0     + data[n - 2][col].1;
            let prev_col = data[n - 1][col - 1].0 + data[n - 1][col - 1].1;
            data[n - 1][col].0 = min(prev_row, prev_col);
        }
        for row in (1..n - 1).rev() {
            let next_row = data[row + 1][col].0 + data[row + 1][col].1;
            let prev_row = data[row - 1][col].0 + data[row - 1][col].1;
            let prev_col = data[row][col - 1].0 + data[row][col - 1].1;
            println!("col = {} row = {} nr = {:4} pr = {:4} pc = {:4}", col, row, next_row, prev_row, prev_col);
            data[row][col].0 = min(min(next_row, prev_row), prev_col);
        }
        {
            let next_row = data[1][col].0     + data[1][col].1;
            let prev_col = data[0][col - 1].0 + data[0][col - 1].1;
            data[0][col].0 = min(next_row, prev_col);
        }
    }
    
    for row in 0..n {
        for col in 0..n {
            print!("{:10?} ", data[row][col]);
        }
        println!("");
    }
    
    return data.iter().fold(data[0][n - 1].0 + data[0][n - 1].1, |acc, ref vec| min(acc, vec.last().unwrap().0 + vec.last().unwrap().1));
}