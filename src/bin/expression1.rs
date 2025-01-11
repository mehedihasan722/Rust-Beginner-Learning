fn print_message(is_greater_than_100: bool) {
    match is_greater_than_100 {
        true => println!("The value is greater than 100"),
        false => println!("The value is not greater than 100"),      
    }
}

fn main() {
    let value: i32 = 105;
    let is_greater_than_100: bool = value > 100;
    print_message(is_greater_than_100);
}