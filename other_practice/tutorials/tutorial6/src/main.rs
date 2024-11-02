use std::io;

fn main() {
    //initializing numbers with different datatypes 
    let x = 255.0f32; // 0 to 255
    let y = 10.0_f64; // -128 to 127
    //converting a number that's too small to a bigger data type
    let z = (x as f64) / y;
    println!("{}",z);
    
    // taking a string input and turning it into a number
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("expected a number");
    let int_input: f64 = input.trim().parse().unwrap();
    println!("{}",int_input + 2.0);
}
