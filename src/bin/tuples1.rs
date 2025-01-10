fn coordinates() -> (i32, i32) {
    (2, 3)
}
fn main() {
    let (_ , y) = coordinates(); 
    if y > 5 {
        println!("Y is greater than 5");
    } else if y < 5{
        println!("Y is not greater than 5");
    } else {
        println!("Y is equal to 5");
    }
}