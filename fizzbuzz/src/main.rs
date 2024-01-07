use std::io;
fn main() {
    let mut usr_input: String = String::new();

    println!("Enter a number");
    io::stdin().read_line(&mut usr_input).expect("Failed to read message");
    
    let num: f32 = usr_input.trim().parse().expect("Invalid input");

    fizzbuzz(num);
}

fn fizzbuzz(value: f32)  {
    let fizz: f32 = 5.0;
    let buzz: f32 = 3.0;

    if value%fizz == 0.0 && value%buzz == 0.0 {
        println!("fizzbuzz");
    }else if value%fizz == 0.0{
        println!("fizz");
    }else if value%buzz == 0.0{
        println!("buzz");
    }else{
        println!("not a fizz, buzz or a fizzbuzz");
    }
    
}