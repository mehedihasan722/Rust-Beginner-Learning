struct GroceryItem {
    quantity: i32,
    id: i32,
}
fn display_quantity(item: &GroceryItem) {
    println!("The item has a quantity of {}", item.quantity);
}
fn display_id(item: &GroceryItem) {
    println!("The item has an id of {}", item.id);
}
fn main() {
    let item: GroceryItem = GroceryItem {
        quantity: 5,
        id: 100,
    };
    display_quantity(&item);
    display_id(&item);
}