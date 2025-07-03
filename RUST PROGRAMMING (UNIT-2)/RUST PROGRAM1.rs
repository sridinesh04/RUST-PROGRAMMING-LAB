fn main() {
    let x: i32 = 10;
    let y: f64 = 5.5;

    let result = x as f64 + y;
    println!("Result of {} + {} = {}", x, y, result);

    let a: f64 = 15.67;
    let b: &str = "123";

    let a_casted: i32 = a as i32;
    println!("Explicit casting of {} to i32: {}", a, a_casted);

    let b_casted: i32 = b.parse().unwrap();
    println!("Explicit casting of {} to i32: {}", b, b_casted);

    let b_to_float: f64 = b.parse().unwrap();
    println!("Explicit casting of {} to f64: {}", b, b_to_float);
}
