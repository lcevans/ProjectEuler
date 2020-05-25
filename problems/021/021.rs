fn divisor_sum(n : i64) -> i64 {
    let mut sum = 0;
    for i in 1..n {
        if n % i == 0 {
            sum += i;
        }
    }
    return sum;
}

fn main() {
    let mut amicable_sum = 0;
    for a in 1..10000 {
        let b = divisor_sum(a);
        if a != b && a == divisor_sum(b) {
            amicable_sum += a;
        }
    }

    println!("{}", amicable_sum);
}
