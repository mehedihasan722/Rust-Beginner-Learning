enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}

fn main() {
    let n = 3;
    match n {
        3 => println!("n is three"),
        other => println!("number : {}", other),
    }
    let flat: Discount = Discount::Flat(100);
    match flat {
        Discount::Flat(amount) => println!("Flat discount of {}", amount),
        Discount::Flat(100) => println!("Percent discount of 100%",),
        _ => println!("Unknown discount type"),
    }
    let ticket = Ticket {
        event: "Concert".to_string(),
        price: 50,
    };

    match ticket {
        Ticket { event, price } if price < 100 => {
            println!("Event: {}, Price: {}", event, price);
        }
        Ticket { event, price } => {
            println!("Event: {}, Price: {}", event, price);
        }
    }
}
