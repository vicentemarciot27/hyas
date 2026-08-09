use std::io;

fn main() {
    println!("================================");
    println!("          THE PILGRIM");
    println!("================================");
    println!();
    println!("You wake beneath a dead tree.");
    println!("A road disappears into the fog.");
    println!();
    println!("[1] Follow the road");
    println!("[2] Enter the forest");
    println!();
    println!("What do you do?");

    let mut input:String = String::new();

    io:: stdin().read_line(&mut input).expect("Failed to read input");

    println!("You chose: {}", input.trim());
}
