fn main() {
    for i in 1..=5 {
        for _ in 0..i {
            print!("&");
        }
        println!();
    }
}
