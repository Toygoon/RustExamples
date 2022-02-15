use std::io;

fn main() {
    println!("Let's guess a number.");
    println!("Input a number.");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Couldn't read a number.");

    println!("Input value : {}", guess);
}