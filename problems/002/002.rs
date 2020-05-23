fn main() {
    let mut a = 1;
    let mut b = 1;
    let mut total = 0;

    while b < 4000000 {
        let tmp = b;
        b = a+b;
        a = tmp;
        if b % 2 == 0 {
            total += b;
        }
    }
    println!("{}", total);
}
