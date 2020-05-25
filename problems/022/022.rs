use std::fs;

fn name_score(name : &str) -> u32 {
    return name.chars().map(|c| c as u32 - 64).sum();
}

fn main() {
    let file_contents = fs::read_to_string("p022_names.txt").unwrap();
    let mut total_score = 0;
    let mut names : Vec<&str> = file_contents.split(",").collect();
    names.sort();
    for (i, quoted_name) in names.iter().enumerate() {
        let name  = &quoted_name[1..quoted_name.len()-1];
        total_score += (i as u32 +1) * name_score(name);
    }
    println!("{}", total_score);
}
