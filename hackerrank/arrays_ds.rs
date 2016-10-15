use std::io;

fn main() {
    let mut str_t = String::new();
    io::stdin().read_line(&mut str_t).unwrap();
    let num: usize = str_t.trim().parse().unwrap();
    str_t.clear();
    
    io::stdin().read_line(&mut str_t).unwrap();
    let arr: Vec<i32> = str_t.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();
    
    for i in 0..num {
        print!("{} ", arr[num - i - 1]);
    }
}
