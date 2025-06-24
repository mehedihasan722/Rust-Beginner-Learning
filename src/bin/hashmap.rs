use std::collections::HashMap;

struct Contents {
    content: String,
}

fn main() {
    let mut lockers: HashMap<i32, Contents> = HashMap::new();
    lockers.insert(
        1,
        Contents {
            content: "Books".to_string(),
        },
    );
    lockers.insert(
        2,
        Contents {
            content: "Bags".to_string(),
        },
    );
    lockers.insert(
        3,
        Contents {
            content: "Shoes".to_string(),
        },
    );

    for (locker_number, content) in lockers.iter() {
        println!("Locker {:} contains: {:}", locker_number, content.content);
    }
}
