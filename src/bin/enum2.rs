enum Color {
    Red,
    Green,
    Blue,
}
fn print_color(color: Color) {
    match color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }
}
fn main() {
    let color: Color = Color::Red;
    print_color(color);
    let color: Color = Color::Green;
    print_color(color);
    let color: Color = Color::Blue;
    print_color(color);
}