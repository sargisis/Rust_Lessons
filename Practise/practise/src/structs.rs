use std::io;

struct Solutions {}

impl Solutions {
    fn program1() {
        let mut guess_number = String::new();

        io::stdin().read_line(&mut guess_number).unwrap_or(0);

        let n_choice = guess_number
            .trim()
            .parse::<i32>()
            .expect("Enter Your number");
        println!("Please Enter your number");

        println!("Your Enter your number is: {n_choice}");
    }

    fn program2() {
        let mut number = String::new();

        io::stdin().read_line(&mut number).unwrap();

        let float_number: Option<f32> = match number.trim().parse::<f32>() {
            Ok(num) => Some(num),
            Err(_) => None,
        };

        if let Some(value) = float_number {
            println!("This value is float_number: {value}");
        } else {
            panic!("This value isn't float_number");
        }
    }

    fn program3() {
        let mut number = String::new();
        let mut number1 = String::new();

        println!("Please Enter your first number");

        io::stdin().read_line(&mut number).unwrap();

        println!("Please Enter your second number");

        io::stdin().read_line(&mut number1).unwrap();

        let num1: u16 = number
            .trim()
            .parse::<u16>()
            .expect("Please Enter correct number");
        let num2: u16 = number1
            .trim()
            .parse::<u16>()
            .expect("Please Enter second number correct");

        let total: u16 = num1 + num2;

        println!("Your result is: {total}");
    }

    fn program4() {
        let mut person = String::new();

        println!("Please Enter your age");
        io::stdin().read_line(&mut person).unwrap();

        let age = person.trim().parse::<u16>().expect("Please Enter your age");

        println!("Your are {age} years old.");
    }

    fn program5() {
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
}
