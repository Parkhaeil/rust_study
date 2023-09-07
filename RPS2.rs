use std::io;
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");

    let mut l: Vec<&str> = input.split_whitespace().collect();
    l.sort();

    let mut cnt = 0;
    let mut chk = HashMap::new();
    chk.insert(String::from("R"), false);
    chk.insert(String::from("P"), false);
    chk.insert(String::from("S"), false);

    for &i in &l {
        if let Some(value) = chk.get_mut(i) {
            if !*value {
                *value = true;
                cnt += 1;
            }
        }
    }

    let result = if cnt == 1 || cnt == 3 {
        "D"
    } else {
        if *chk.get("R").unwrap() && *chk.get("P").unwrap() {
            "P"
        } else if *chk.get("R").unwrap() && *chk.get("S").unwrap() {
            "R"
        } else {
            "S"
        }
    };

    println!("{}", result);
}

