use rand::Rng;

mod random_number;

use random_number::RandomGenerator;


    pub fn generate() -> RandomGenerator {
        super::shared();
        let number = rand::thread_rng().gen_range(0..=100);
            RandomGenerator::new(number)
    }

    pub fn other_fn() {
        println!("Hello world!");
    }
   
