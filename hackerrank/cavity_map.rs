use std::io;

fn main() {
  let mut str_t = String::new();

  io::stdin().read_line(&mut str_t).unwrap();
  let n: usize = str_t.trim().parse().unwrap();
  str_t.clear();

  let mut mapd: Vec<Vec<u8>> = Vec::with_capacity(n);
  for _ in 0..n {
    io::stdin().read_line(&mut str_t).unwrap();
    let tvec: Vec<u8> = str_t.trim().as_bytes().to_vec();
    str_t.clear();
    mapd.push(tvec);
  }

  let sx: u8 = 255;

  for x in 1..(n - 1) {
    for y in 1..(n - 1) {
      if (mapd[x][y] > mapd[x - 1][y]) &&
         (mapd[x][y] > mapd[x][y - 1]) &&
         (mapd[x][y] > mapd[x + 1][y]) &&
         (mapd[x][y] > mapd[x][y + 1]) {
           mapd[x][y] = sx;
         }
    }
  }
  for x in 1..(n - 1) {
    for y in 1..(n - 1) {
      if mapd[x][y] == sx {
        mapd[x][y] = b'X';
      }
    }
  }
  for y in 0..n {
    let tvec: Vec<u8> = mapd[y].clone();
    let str_tmp = String::from_utf8(tvec).unwrap();
    println!("{}", str_tmp);
  }
}

