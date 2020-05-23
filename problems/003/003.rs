use std::cmp;

fn largest_prime_factor(x : i64) -> i64 {
    for i in 2..((x as f64).sqrt() as i64 + 1) {
        if x % i == 0 {
            // Found divisor i
            return cmp::max(i, largest_prime_factor(x / i));
        }
    }
    // x was prime
    return x;
}


fn main() {
    println!("{}", largest_prime_factor(600851475143));
}
