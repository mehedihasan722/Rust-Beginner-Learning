fn main() {
    let mut counter: i32 = 0;
    while counter < 10 {
        counter += 1;
        println!("Counter is {}", counter);
    }
    print!("Loop ended");
}