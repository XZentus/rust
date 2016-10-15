use std::io;

fn main() {
  let mut str_t = String::new();
  io::stdin().read_line(&mut str_t).unwrap();

  let ncase: usize = str_t.trim().parse().unwrap();
  str_t.clear();

  for _ in 0..ncase {
    io::stdin().read_line(&mut str_t).unwrap();
    let mut nk: Vec<i32> = str_t.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();
    str_t.clear();

    io::stdin().read_line(&mut str_t).unwrap();
    nk[1] -= str_t.split_whitespace().map(|x| x.trim().parse::<i32>().unwrap())
         .fold(0, |accum, x| if x <= 0 {accum + 1} else {accum});
    str_t.clear();

    if nk[1] > 0 {
      println!("YES");
    }
    else {
      println!("NO");
    }
  }
}
