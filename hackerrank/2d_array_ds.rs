use std::io;

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

const W: usize = 6;
const H: usize = 6;

const PATTERN_SIZE: usize = 7;
const PATTERN: [Point; PATTERN_SIZE] = [Point {x: 0, y: 0}, Point {x: 1, y: 0}, Point {x: 2, y: 0},
                                                            Point {x: 1, y: 1},
                                        Point {x: 0, y: 2}, Point {x: 1, y: 2}, Point {x: 2, y: 2}];

fn main() {
    let mut input_line = String::new();
    let mut map: Vec<Vec<i32>> = Vec::with_capacity(H);
    for _ in 0..6 {
        io::stdin().read_line(&mut input_line).unwrap();
        map.push(input_line.split_whitespace().map(|x| x.trim().parse().unwrap()).collect());
        input_line.clear();
    }
    let max_sum = calc_pattern(&map);
    println!("{}", max_sum);
}

fn calc_pattern(map: &Vec<Vec<i32>>) -> i32 {
    let (max_dx, max_dy) = pattern_max();
    let mut max_sum = std::i32::MIN;
    for line in 0..(W - max_dx) {
        for col in 0..(H - max_dy) {
            let m_s = get_max_sum(map, line, col);
            if m_s > max_sum {max_sum = m_s};
        }
    }
    max_sum
}

fn get_max_sum(map: &Vec<Vec<i32>>, line: usize, col: usize) -> i32 {
    let mut sum = 0;
    for i in 0..PATTERN_SIZE {
        let i = PATTERN[i];
        sum += map[line + i.y][col + i.x]
    }
    sum
}

fn pattern_max() -> (usize, usize) {
    let mut dx = 0;
    let mut dy = 0;
    for i in 0..PATTERN_SIZE {
        let i = PATTERN[i];
        if i.x > dx {dx = i.x};
        if i.y > dy {dy = i.y};
    }
    (dx, dy)
}
