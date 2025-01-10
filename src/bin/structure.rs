struct GroceryItem {
    name: String,
    price: f64,
    quantity: u32,
}

fn main() {
    let item: GroceryItem = GroceryItem {
        name: "Milk".to_string(),
        price: 3.50,
        quantity: 2,
    };

    println!("Item: {}", item.name);
    println!("Price: ${}", item.price);
    println!("Quantity: {}", item.quantity);
}