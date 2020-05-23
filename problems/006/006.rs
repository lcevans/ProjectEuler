// Could do with math but the program is trivial...
fn main() {
    let mut sum_squares = 0;
    let mut sum = 0;
    for i in 1..101 {
        sum_squares += i * i;
        sum += i;
    }

    println!("{}", sum * sum - sum_squares);
}
