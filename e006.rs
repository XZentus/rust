fn main() {
    let mut sq_sum: u64 = 1;
    let mut sum_sq: u64 = 1;

    for i in 2..=100 {
        sq_sum += i;
        sum_sq += i*i;
    }
    sq_sum *= sq_sum;
    println!("{} - {} = {}", sq_sum, sum_sq, sq_sum - sum_sq);
}
