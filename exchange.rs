use std::io;

fn main() {
    let mut hwanyul = String::new();
    let mut money = String::new();

    io::stdin().read_line(&mut hwanyul).expect("Failed to read line");
    io::stdin().read_line(&mut money).expect("Failed to read line");

    let hwanyul: f64 = hwanyul.trim().parse().expect("Please enter a valid integer");
    let money: f64 = money.trim().parse().expect("Please enter a valid integer");

    let mut euro = money / hwanyul;

    if euro < 100.0{
        euro = euro - 1.0;
    }

    let hundred = (euro / 100.0) as i32;
    let fifty = ((euro % 100.0) / 50.0) as i32;
    let ten = ((euro % 100.0 % 50.0) / 10.0) as i32;
    let unit = ((euro % 100.0 % 50.0 % 10.0) / 1.0) as i32;

    println!("{} {} {} {}", hundred, fifty, ten, unit);
}
