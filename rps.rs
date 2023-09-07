use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");

    let mut lst: Vec<&str> = input.split_whitespace().collect();

    lst.sort();

    let result = if lst[0] == lst[1] {
        "D"
    } else if lst[0] == "P" {
        if lst[1] == "R" {
            "P"
        }
        else{
            "S"
        }

    } else {
        "R"
    };

    println!("{}", result);

}
