#[derive(Debug)]
struct Range<T> {
    start: T,
    end: T,
    step: T,
}

impl Range<u64> {
    fn new(start: u64, end: u64, step: u64) -> Range<u64> {
        Range {
            start: next_div(start, step),
            end: prev_div(end, step),
            step,
        }
    }

    fn num_values(&self) -> u64 {
        1 + (self.end - self.start) / self.step
    }

    fn range_sum(&self) -> u64 {
        let num_values = self.num_values();

        if num_values % 2 == 0 {
            (num_values - 1) * mid_value(self.start + self.step, self.end) + self.start
        }
        else {
            num_values * mid_value(self.start, self.end)
        }
    }
}

fn mid_value(value1: u64, value2: u64) -> u64 {
    (value1 + value2) / 2
}

fn next_div (n: u64, d: u64) -> u64 {
    if n % d == 0 {
        n
    }
    else {
        n + d - n % d
    }
}

fn prev_div (n: u64, d: u64) -> u64 {
    n - n % d
}

fn main() {
    let beg: u64 = 1;
    let end: u64 = 999;

    let range3: Range<u64> = Range::new(beg, end, 3);
    let range5: Range<u64> = Range::new(beg, end, 5);
    let range15: Range<u64> = Range::new(beg, end, 15);

    println!("{}", range3.range_sum() + range5.range_sum() - range15.range_sum());
}
