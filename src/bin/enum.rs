fn main() {
    enum Age {
        UnderAge,
        Adult,
        Child,
    }

    let age: i32 = 3;
    let person: Age = if age >= 21 {
        Age::Adult
    } else if age < 21 && age >= 18 {
        Age::UnderAge
    } else {
        Age::Child
    };

    match person {
        Age::Adult => println!("You are old enough to drink"),
        Age::UnderAge => println!("You are not old enough to drink"),
        Age::Child => println!("You are a child"),
    }
}