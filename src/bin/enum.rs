fn main() {
    enum Age {
        UnderAge,
        Adult,
    }

    let age: i32 = 30;
    let person: Age = if age >= 21 {
        Age::Adult
    } else {
        Age::UnderAge
    };

    match person {
        Age::Adult => println!("You are old enough to drink"),
        Age::UnderAge => println!("You are not old enough to drink"),
    }
}