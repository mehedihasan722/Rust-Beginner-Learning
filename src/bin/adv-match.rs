use std::vec;

enum Ticket {
    Standard(f64),
    Backstage(f64, String),
    Vip(String, f64),
}

fn main() {
    let tickets: Vec<Ticket> = vec![
        Ticket::Standard(50.0),
        Ticket::Backstage(100.0, "VIP Lounge".to_string()),
        Ticket::Vip("Front Row".to_string(), 200.0),
    ];
    for ticket in tickets {
        match ticket {
            Ticket::Standard(price) => println!("Standard ticket: ${}", price),
            Ticket::Backstage(price, area) => {
                println!("Backstage ticket: ${}, Area: {}", price, area)
            }
            Ticket::Vip(area, price) => println!("VIP ticket: Area: {}, Price: ${}", area, price),
        }
    }
}
