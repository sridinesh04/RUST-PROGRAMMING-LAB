fn main() {
    let x = 10;
    let y = &x;

    println!("Value of x: {}", x);
    println!("Borrowed value of x (using reference y): {}", y);

    let z = Box::new(20);
    let w = &z;

    println!("Value of z (Boxed): {}", z);
    println!("Borrowed value of z (using reference w): {}", *w);
}
