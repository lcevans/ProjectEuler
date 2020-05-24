use std::cmp;

static TRIANGLE: &'static str = r#"
75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23
"#;

const SIZE: usize = 15; // Triangle size

fn main() {
    // Build triangle array
    let mut triangle_array = [[0u32; SIZE]; SIZE]; // SIZE x SIZE of u32
    for (i, line) in TRIANGLE[1..TRIANGLE.len()-1].split('\n').enumerate() {
        for (j, num) in line.split(" ").enumerate() {
            triangle_array[i][j] = num.parse().unwrap();
        }
    }

    // Fill max_path via dynamic programming
    let mut max_path = [[0u32; SIZE]; SIZE];
    for j in 0..SIZE {
        max_path[SIZE-1][j] = triangle_array[SIZE-1][j];
    }

    for i in (0..SIZE-1).rev() {
        for j in 0..i+1 {
            max_path[i][j] = triangle_array[i][j] + cmp::max(max_path[i+1][j], max_path[i+1][j+1]);
        }
    }

    println!("{}", max_path[0][0]);
}
