struct LineItem {
    name: String,
    count: i32,
}

fn print_name(item: &str) {
    println!("Item name: {}", item);
}
fn main() {
    let receipt: Vec<LineItem> = vec![
        LineItem {
            name: "Apples".to_string(),
            count: 3,
        },
        LineItem {
            name: "Bananas".to_string(),
            count: 2,
        },
        LineItem {
            name: String::from("Cherries"),
            count: 5,
        },
    ];

    for item in receipt {
        print_name(&item.name);
        println!("Item: {}, Count: {}", item.name, item.count);
    }
}
