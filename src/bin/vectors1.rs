fn main() {
    let my_numbers: Vec<i32> = vec![10, 20, 30, 40, 50];
    for num in &my_numbers {
        match num {
            30 => println!("Found the number 30!"),
            _ => println!("it is {}", num),
        }
    }

    println!("The length of the vector is {}", my_numbers.len());
}
