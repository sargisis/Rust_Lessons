use std::{convert, io}; 

const C: f32 = 32.0; 



fn c_to_f(celsus_temp: f32) -> f32 {
    (celsus_temp * (9.0 / 5.0)) + C
}

fn f_to_c(fahrinhetit_temp: f32) -> f32 {
    (fahrinhetit_temp - C) * (5.0 / 9.0)
} 

fn Convert(temperature: f32 , choice: u8) -> Option<f32> {
    match choice {
        1 => Some(c_to_f(temperature)),
        2 => Some(f_to_c(temperature)),
        _ => None, 
    }
}

fn main() {
    println!("Temperature Converter. \n (1) C to F \n (2) F to C");

    let mut user_choice = String::new();


    io::stdin().read_line(&mut user_choice).unwrap();

    let n_choice = user_choice
        .trim()
        .parse::<u8>()
        .expect("Please Type a number");

    println!("Enter Temperature");

    let mut temperature = String::new();


    io::stdin().read_line(&mut temperature).unwrap();

    let temperature = temperature
        .trim()
        .parse::<f32>()
        .expect("Plase Type a number");


    match Convert(temperature, n_choice) {
        Some(Result) => println!("The convertion is: {Result} succesed "),
        None => println!("Unkonwn convertation requested"),
     }

}