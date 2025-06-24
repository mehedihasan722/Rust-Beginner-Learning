#[derive(Debug)]
struct Adult {
    age: u8,
    name: String,
}

impl Adult {
    fn new(age: u8, name: String) -> Result<Self, String> {
        if age < 18 {
            Err(format!("{:} is not an adult", name))
        } else {
            Ok(Self { age, name })
        }
    }
}

fn main() {
    let adults: Vec<Result<Adult, String>> = vec![
        Adult::new(20, "Alice".to_string()),
        Adult::new(17, "Bob".to_string()),
        Adult::new(30, "Charlie".to_string()),
    ];

    for adult in adults {
        match adult {
            Ok(adult) => println!("{:} is an adult.", adult.name),
            Err(e) => println!("{:}", e),
        }
    }
}
