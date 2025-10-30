#![warn(clippy::all, clippy::pedantic)]
#[derive(Debug)]

enum OrderStatus {
    Paid { amount: i32 },
    Sent,
    Delivered,
    Disputed(String),
}
impl OrderStatus {
    fn info(&self) {
        match self {
            Self::Paid { amount } => println!("Paid, {amount}"),
            Self::Sent => println!("Sent Order"),
            Self::Delivered => println!("Delivered"),
            Self::Disputed(reason) => println!("Disputed with reason: {reason}"),
        }
    }
}
#[derive(Debug)]
struct Order {
    customer: String,
    status: OrderStatus,
}
fn main() {
    let status = OrderStatus::Sent;
    demo(&status);

    let order = Order {
        customer: String::from("John Doe"),
        status,
    };

    let value: Option<i8> = None;

    println!("{value:#?}");

    // let result = value.unwrap_or(0) + 5;
    // println!("{result}");

    // match value {
    //     Some(a) => {
    //         let result = a + 5;
    //         println!("{result}");
    //     },
    //     None => (),
    // }

    if let Some(a) = value {
        let result = a + 5;
        println!("{result}");
    } else {
        println!("Error");
    }
}

fn demo(status: &OrderStatus) {
    println!("{status:#?}");
}