#[derive(Debug)]
enum Access {
    Full,
}
fn one_two_three() -> (i32, i32, i32) {
    (1, 2, 3)
}
fn main() {
    let access: Access = Access::Full;
    let numbers: (i32, i32, i32) = one_two_three();
    let (one, two, three) = one_two_three();
    println!("Access: {:?}", access);
    println!("Numbers: {:?}", numbers); 
    println!("One: {}", one);
    println!("Two: {}", two);
    println!("Three: {}", three);
}