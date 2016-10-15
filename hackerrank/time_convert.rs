use std::io;

fn main() {
    let mut str_t = String::new();
    
    io::stdin().read_line(&mut str_t).unwrap();
    
    let mut v: Vec<char> = str_t.chars().collect();
    if v[8] == 'P' {
        let hh : u8 = ((v[0] as u8) - ('0' as u8)) * 10 + 
                      ((v[1] as u8) - ('0' as u8)) + 12;
    
        v[0] = ((hh / 10) % 10 + ('0' as u8)) as char;
        v[1] = (hh % 10 + ('0' as u8)) as char;
    }
    v.truncate(8);
    str_t.clear();
    for c in v {str_t.push(c);}
    
    println!("{}", str_t);
}