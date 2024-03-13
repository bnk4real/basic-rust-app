// if-else aand operations

pub fn cal(x:i32, y:i32) -> i32 {
    // AND &&
    // OR ||
    // NOT !=
    if x == 0 && y == 0 {
        println!("Number must be greater than zero.");
    } else if x > y {
        println!("X must not greater than y.");
    } else {
        println!("ERROR!!!");
    }

    return x + y
}