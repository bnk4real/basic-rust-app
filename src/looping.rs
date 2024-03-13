pub fn looping(){
    // for loops
    let data: [&str; 3] = ["Asus", "MSI", "Mac"];
    for x in data {
         println!("{}", x);
    }
    // while loop
    let mut z: i32 = 1;
    while z <= 50 {
        println!("Number of increment: {}", z);
        z += 1
    }
} 