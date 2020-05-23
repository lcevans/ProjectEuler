use std::cmp;

fn is_palindrome(x: i32) -> bool {
    let s = x.to_string();
    let r : String = s.chars().rev().collect();
    return s == r;
}



fn main() {
    let mut largest = 1;
    for i in 1..1000 {
        for j in 1..1000 {
            if is_palindrome(i*j) {
                largest = cmp::max(largest, i*j);
            }
        }
    }
    println!("{}", largest);
}
