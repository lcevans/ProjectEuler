fn main() {
    let mut last_prime = 0;
    let mut num_primes = 0;
    let mut n = 2; // number to check
    while num_primes < 10001 {

        let mut is_prime = true;
        for i in 2..((n as f64).sqrt() as i64) + 1 {
            if n % i == 0 {
                is_prime = false;
            }
        }

        if is_prime {
            num_primes += 1;
            last_prime = n;
        }

        n += 1;
    }
    println!("{}", last_prime);
}
