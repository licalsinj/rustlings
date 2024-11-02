fn main() {
    print_hello_world("World".to_string(), "Word 2".to_string());
}
fn print_hello_world(_words: String, word2: String) {
    println!("{} Hello {}", word2, word2);
}
