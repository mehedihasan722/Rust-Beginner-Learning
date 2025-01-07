fn main() {
    let a = 2;
    let b = 3;
    let sum = add(a, b);
    display_result(a, b, sum);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn display_result(a: i32, b: i32, sum: i32) {
    println!("The sum of {} and {} is {}", a, b, sum);
}