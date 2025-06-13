struct Person {
    name: String,
    favorite_color: String,
    age: i32,
}

fn print_person_info(person: &str) {
    println!("Data = {}", person);
}

fn main() {
    let people: Vec<Person> = vec![
        Person {
            name: "Alice".to_string(),
            favorite_color: "Blue".to_string(),
            age: 30,
        },
        Person {
            name: "Bob".to_string(),
            favorite_color: "Green".to_string(),
            age: 25,
        },
        Person {
            name: "Charlie".to_string(),
            favorite_color: "Red".to_string(),
            age: 35,
        },
    ];

    for person in &people {
        if person.age <= 30 {
            print_person_info(&person.name);
            print_person_info(&person.favorite_color);
        }
    }
}
