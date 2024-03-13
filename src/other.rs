pub fn other() {
    // call function
    let s: String = String::from("meeeee.");
    borower(s);
}

fn borower(s: String) {
    println!("{}", s);
}