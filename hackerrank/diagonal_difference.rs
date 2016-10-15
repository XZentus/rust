use std::io;

fn main() {
    
    let mut str_t = String::new();
    io::stdin().read_line(&mut str_t).unwrap();
    let num: u32 = str_t.trim().parse().unwrap();
    str_t.clear();
    
    let mut matrix: Vec<Vec<i32>> = Vec::with_capacity(num as usize);
    
    for _ in 0..num {
      io::stdin().read_line(&mut str_t).unwrap();
      let tvec : Vec<i32> = str_t.split_whitespace().map(|x| x.parse().unwrap()).collect();
      str_t.clear();
      matrix.push(tvec);
    }

    let mut sum1: i32 = 0;
    let mut sum2: i32 = 0;

    for i in 0..num {
      sum1 += matrix[i as usize][i as usize];
      sum2 += matrix[(num - i - 1) as usize][(i as usize)];
    }

    println!("{}", i32::abs(sum1 - sum2));
}
