use std::collections::HashMap;


fn num_divisors(num : i64) -> i64 {
    // Get prime factorization
    let mut n = num;
    let mut prime_divs : HashMap<i64, i64> = HashMap::new();
    loop {
        let mut found_div = false;
        for i in 2..((n as f64).sqrt() as i64 + 1) {
            if n % i == 0 {
                found_div = true;
                match prime_divs.get(&i) {
                    Some(&val) => {prime_divs.insert(i, val+1);}
                    None => {prime_divs.insert(i, 1);}
                }
                n /= i;
                break;
            }
        }

        if !found_div {
            match prime_divs.get(&n) {
                Some(&val) => {prime_divs.insert(n, val+1);}
                None => {prime_divs.insert(n, 1);}
            }
            break;
        }
    }

    return prime_divs.values().map(|v| v + 1).product();
}


fn main() {
    let mut k = 1;
    let mut triangle = 1;

    while num_divisors(triangle) < 500 {
        k += 1;
        triangle += k;
    }
    println!("{}", triangle);
}
