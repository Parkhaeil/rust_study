use std::io;

fn main() {
    let mut shoot = String::new();
    io::stdin().read_line(&mut shoot).expect("input failed");

    let mut score = 1;
    let mut total_score = 0;
    for c in shoot.chars() {
    if c == 'h' {
            total_score += score;
            score += 1;
        } else {
            total_score -= 1;
            score = 1;
        }
    }
    println!("{}", total_score);
}
