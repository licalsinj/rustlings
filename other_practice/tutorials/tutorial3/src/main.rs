fn main() {
    const SECONDS_IN_MINUTE: u32 = 60;
    println!("{}", SECONDS_IN_MINUTE);
    let mut x = 4;
    println!("x is: {}",x);
    {
        let x = x - 2;
        println!("x is: {}",x);
    }
    x = x + 1;
    println!("x is: {}",x);
}
