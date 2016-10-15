use std::thread;
use std::sync::{Mutex, Arc};

struct Table {
    forks: Vec<Mutex<()>>,
}

struct Phil {
    name:  String,

    left:  usize,
    right: usize,
}

impl Phil {
    fn new(name: &str, left: usize, right: usize) -> Phil {
        Phil {
            name:  name.to_string(),
            left:  left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        let _left  = table.forks[self.left ].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();
        println!("{} начал есть.", self.name);

        thread::sleep_ms(1000);

        println!("{} закончил есть.", self.name);
    }
}

fn main() {
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let phls = vec! [
        Phil::new("p1", 0, 1),
        Phil::new("p2", 1, 2),
        Phil::new("p3", 2, 3),
        Phil::new("p4", 3, 4),
        Phil::new("p5", 0, 4),
    ];

    let handles: Vec<_> = phls.into_iter().map(|p| {
        let table = table.clone();

        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
