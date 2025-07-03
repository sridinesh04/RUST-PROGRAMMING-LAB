fn main() {
    let x = 5; // x is declared and initialized in the main function's scope

    println!("Value of x in main scope: {}", x);

    {
        let x = 10; // This x shadows the previous x in the inner scope
        println!("Value of x in inner scope: {}", x);
    }

    // After the inner scope ends, the shadowed x no longer exists
    println!("Value of x after inner scope: {}", x);

    // Shadowing can also change the type
    let x = "Hello, Rust!"; // Shadowing x to change it from an integer to a string
    println!("Value of x after shadowing to a string: {}", x);
}
