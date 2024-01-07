use std::io;
fn main() {
    println!("Hello, world!");
    let mut usr_input = String::new();

    io::stdin().read_line(&mut usr_input).expect("failed to read");
    println!("{usr_input}")
}
