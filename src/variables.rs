pub fn vars() {
    // by default Rust variable will be assigned as inmutable
    let mut x: i32 = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}