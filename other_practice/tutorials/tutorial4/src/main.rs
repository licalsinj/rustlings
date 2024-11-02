fn main() {
    // Primitive data types - singular piece of data 
    let unsigned_int: u8 = 255;
    let signed_int: i8 = -127;
    let boo = false;
    let charmander = 'c';
    // Scalar data types
    // Tuple - can be a combination of multiple types
    let mut tup: (u8,i8,bool,char) = (unsigned_int,signed_int,boo,charmander);
    tup.0 = 128;
    println!("{}",tup.3);
    // Array must be of the same type
    let arr: [i32;5] = [1,2,3,4,5];
    println!("First index is: {}",arr[0]);
}
