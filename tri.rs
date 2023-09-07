use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut l: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse"))
        .collect();
    l.sort();

    let result = if l[2] >= l[1] + l[0] {
        0
    } else {
        let a_squared = l[0].pow(2);
        let b_squared = l[1].pow(2);
        let c_squared = l[2].pow(2);
        
        if c_squared == a_squared + b_squared {
            1
        } else if c_squared > a_squared + b_squared {
            2
        } else {
            3
        }
    };
    println!("{}", result);

}
