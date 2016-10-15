use std::io;

fn main() {
  let mut str_t = String::new();

  io::stdin().read_line(&mut str_t).unwrap();
  let mut n: usize = str_t.trim().parse().unwrap();

  for _ in 0..n {
    runtest();
  }
}

fn runtest() {
  let mut str_t = String::new();
  io::stdin().read_line(&mut str_t).unwrap();
  let matrix_xy: Vec<i32> = str_t.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();
  str_t.clear();

  let mut matrix: Vec<Vec<u8>> = Vec::new();
  read_matrix(&mut matrix, matrix_xy[0]);


  io::stdin().read_line(&mut str_t).unwrap();
  let template_xy: Vec<i32> = str_t.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();
  str_t.clear();

  let mut template: Vec<Vec<u8>> = Vec::new();
  read_matrix(&mut template, template_xy[0]);

  find_template(&matrix, matrix_xy[0], matrix_xy[1], &template, template_xy[0], template_xy[1]);
}

fn read_matrix(matrix: &mut Vec<Vec<u8>>, my: i32) {
  let mut str_t = String::new();
  for _ in 0..my {
    io::stdin().read_line(&mut str_t).unwrap();
    let vt: Vec<u8> = str_t.trim().as_bytes().iter().map(|x| x - b'0').collect();
    matrix.push(vt);
    str_t.clear();
  }
}

#[allow(unused_assignments)]
fn find_template(matrix: &Vec<Vec<u8>>, mx: i32, my: i32, template: &Vec<Vec<u8>>, tx: i32, ty: i32) {
  let limx: usize = (mx - tx + 1) as usize;
  let limy: usize = (my - ty + 1) as usize;
  let mut result: bool;

  for posx in 0..limx {
    'tmpchk: for posy in 0..limy {

      result = true;

      for chkx in 0..(tx as usize) {
        for chky in 0..(ty as usize) {
          if matrix[posx + chkx][posy + chky] != template[chkx][chky] {
            result = false;
            continue 'tmpchk;
          }
        }
      }
      if result == true {
        println!("YES");
        return;
      }
    }
  }
  println!("NO");
}
