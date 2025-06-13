fn print_it(data: &str) {
    println!("Hello, world!, {}", data);
}

fn main() {
    print_it("This is a string passed to the function");
    let owned_string: String = "Owned String".to_owned();
    print_it(&owned_string);
    let another_string: String = String::from("Another String");
    print_it(&another_string);
}
