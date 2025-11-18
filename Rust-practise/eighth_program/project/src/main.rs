use std::collections::HashMap;

fn main() {
    let mut a = HashMap::new();

    a.insert(String::from("Demo"), 55);
    a.insert(String::from("Value"), 44);

    a.entry(String::from("demo")).or_insert(50);
    let value = a.get(&String::from("Demo")).unwrap();
    
    for (k, v) in &a {
        println!("Key: {k} , value: {v}");
    }
 }