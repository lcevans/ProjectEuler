fn collatz(mut n : i64) -> i64 {
    let mut length : i64 = 1;
    while n > 1 {
        length += 1;
        if n % 2 == 0 {
            n /= 2;
        }
        else {
            n = 3*n + 1;
        }
    }
    return length;
}


fn main() {
    let mut longest = 1;
    let mut longest_len = 1;
    for n in 1..1000000 {
        let tmp = collatz(n);
        if longest_len < tmp {
            longest = n;
            longest_len = tmp;
        }
    }
    println!("{}", longest);
}
