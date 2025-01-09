fn main() {
    let mut counter: i32 = 0;
    loop {
        counter += 1;
        println!("Counter is {}", counter);
        if counter == 10 {
            break;
        }
    }
}