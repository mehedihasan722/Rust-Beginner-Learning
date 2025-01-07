fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
}

fn mul(a: i32, b: i32) -> i32 {
    a * b
}

fn div(a: i32, b: i32) -> i32 {
    a / b
}

fn rem(a: i32, b: i32) -> i32 {
    a % b
}

fn main() {
    let a = 10;
    let b = 20;

    println!("Sum: {}", add(a, b));
    println!("Sub: {}", sub(a, b));
    println!("Mul: {}", mul(a, b));
    println!("Div: {}", div(a, b));
    println!("Rem: {}", rem(a, b));
}
