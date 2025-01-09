fn main() {
    let variable: i32 = 5;  
    if variable > 5 {
        println!("{} is greater than 5", variable);
    } else if variable < 5 {
        println!("{} is not greater than 5", variable);
    } else {
        println!("{} is equal to 5", variable);
    }
 }