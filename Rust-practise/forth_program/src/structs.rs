#![warn(clippy::all , clippy::pedantic)]

#[derive(Debug)]
struct Car {
    brand: String, 
    max_speed: u16, 
    max_gas: u16 , 
    current_gas: f32, 
    gas_consumption: f32,
}

impl Car {
    fn New(
        brand: &str, 
        max_speed: u16, 
        max_gas: u16 , 
        current_gas: f32, 
        gas_consumption: f32,
    ) -> Self {
        Self { 
            brand: String::from(brand)
            , max_speed
            , max_gas
            , current_gas
            , gas_consumption 
        }
    }


    fn drive(&mut self , distance: f32) {
    let total_gas_consumed = distance * self.gas_per_km();

    if total_gas_consumed > self.current_gas {
        println!("Not enough gas");
    } else {
        println!("Driving");
        self.current_gas -= total_gas_consumed;
    }
    }

    fn gas_per_km(&self) -> f32 {
        self.gas_consumption / 100.0
    }
}

fn main() {
    let mut my_car = Car {
        brand: String::from("Toyota"),
        max_speed: 180,
        max_gas: 50,
        current_gas: 10.0,
        gas_consumption: 0.1,
    };


    println!("Let's drive!");

    let distance: f64 = 40.0;
    // drive(&mut my_car , (distance as f32).into());

    let my_car2 = Car::New("Volvo", 150, 50, 20.3, 33.3);
    println!("{my_car2:#?}");

    let my_car3 = Car {
        gas_consumption: 222.0, 
        ..my_car2
    };

    println!("{my_car3:#?}");
}

// fn drive(car: &mut Car, distance: f32) {
//     let gas_per_km = car.gas_consumption / 100.0; 
//     let total_gas_consumed = distance * gas_per_km;

//     if total_gas_consumed > car.current_gas {
//         println!("Not enough gas");
//     } else {
//         println!("Driving");
//         car.current_gas -= total_gas_consumed;
//     }
// }