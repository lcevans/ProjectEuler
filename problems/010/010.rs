fn main() {
    let mut total = 0;

    for n in 2..2000000 {
        let mut is_prime = true;
        for i in 2..((n as f64).sqrt() as i64 + 1) {
            if n % i == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            total += n
        }
    }

    println!("{}", total);
}
