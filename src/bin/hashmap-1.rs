use std::collections::HashMap;

fn main() {
    let mut stock: HashMap<&'static str, i32> = HashMap::new();
    stock.insert("Chair", 10);
    stock.insert("Table", 5);
    stock.insert("Lamp", 15);
    stock.insert("Sofa", 0);
    stock.insert("Desk", 8);

    let mut total_items: i32 = 0;

    for (item, quantity) in stock.iter() {
        total_items += quantity;
        let stock_count: String = if quantity == &0 {
            "out of stock".to_owned()
        } else {
            format!("{} in stock", quantity)
        };
        println!("{}: {}", item, stock_count);
    }
    println!("Total items in stock: {}", total_items);
}
