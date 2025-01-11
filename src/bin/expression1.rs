fn print_message(is_greater_than_100: bool) {
    match is_greater_than_100 {
        true => println!("The value is greater than 100"),
        false => println!("The value is not greater than 100"),      
    }
}
enum Light {
    Bright,
    Dull,
}

fn display_light(light: &Light) {
    match light {
        Light::Bright => println!("The light is bright"),
        Light::Dull => println!("The light is dull"),
    }
}
fn main() {
    let value: i32 = 105;
    let is_greater_than_100: bool = value > 100;
    print_message(is_greater_than_100);
    let dull_light: Light = Light::Dull;
    display_light(&dull_light);
    display_light(&dull_light);

}