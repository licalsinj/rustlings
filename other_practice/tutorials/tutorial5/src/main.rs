use std::io;
fn main() {
    // User Input
    println!("I am a cave!");
    let mut echo = String::new();
    io::stdin().read_line(&mut echo).expect("failed to read line");
    println!("{}",echo);
    println!("{}",echo);
    println!("{}",echo);
}
