mod prelude {
    pub use std::env as envoriment; 
    pub use std::io;
    use crate::generator::{generate , other_fn};
}

mod generator;

use prelude::*; 
fn main() {
    let random = generator::generate();
    println!("Random number: {}", random.value);

}


pub fn shared() {
    println!("Hello world");
}