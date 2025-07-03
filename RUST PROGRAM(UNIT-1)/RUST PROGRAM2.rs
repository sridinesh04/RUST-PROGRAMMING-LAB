fn main() {
    for i in 1..=5 {
        print!("{:0>width$}\n", format!("{:1$}", i, i));
    }
}
