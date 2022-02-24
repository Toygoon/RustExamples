use std::io;
use rand::Rng;

fn main() {
    println!("Let's guess a number.");
    println!("Input a number.");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("System generated value : {}", secret_number);
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Couldn't read a number.");

    println!("Input value : {}", guess);
}
