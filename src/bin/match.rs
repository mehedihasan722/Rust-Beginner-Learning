fn main() {
    let my_name: &str = "mehedi";
    match  my_name {
        "Jane" => println!("Hello Jane!"),
        "John" => println!("Hello John!"),
        _ => println!("Hello Stranger!"),
    }
 }