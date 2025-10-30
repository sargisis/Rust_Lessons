use std::io;
mod structs;

fn main() {
    let mut number = String::new();

    println!("Please Enter your number");
    io::stdin().read_line(&mut number).unwrap();

    let x = number
        .trim()
        .parse::<i32>()
        .expect("That's not correct number");

    let result: i32 = 4 * x + 21 * x * x - 12;

    println!("Your result is: {result}");
}
