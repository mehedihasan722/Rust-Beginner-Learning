struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let mary: Student = Student {
        name: "Mary".to_string(),
        locker: Some(101),
    };
    println!("Student: {}", mary.name);
    match mary.locker {
        Some(locker_number) => println!("Locker number: {}", locker_number),
        None => println!("No locker assigned"),
    }
}
