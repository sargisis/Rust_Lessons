use std::io; 


fn process(str: &String) -> u8 {
    str.trim().parse::<u8>().expect("Please Enter Your number!")
}

fn main() {
    let mut user_choice: String = String::new();

    io::stdin().read_line(&mut  user_choice).unwrap();

    let n_choice = process(&user_choice);


    println!("Number: {n_choice}");
    println!("String: {user_choice}");
}