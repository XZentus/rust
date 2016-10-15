use std::io;

fn main() {
  let mut str_t = String::new();
  
  io::stdin().read_line(&mut str_t).unwrap();

  let num: usize = str_t.trim().parse().unwrap();

  let mut outp = vec![' ' as u8; num];

  for i in 0..num {
    outp[num - 1 - i] = b'#';
    let tmp = outp.clone();
    println!("{}", String::from_utf8(tmp).unwrap());
  }

}
