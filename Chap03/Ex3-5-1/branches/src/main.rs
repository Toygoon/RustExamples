use std::io;

fn main() {
    println!("Input the number.");
    
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Invalid input.");

    let number: i32 = number.trim().parse().expect("Invalid input");

    if number % 4 == 0 {
        println!("The number is divided into 4.");
    } else if number % 3 == 0 {
        println!("The number is divided into 3");
    } else if number % 2 == 0 {
        println!("The number is divided into 2");
    } else {
        println!("The number doesn't be divided into 2 or 3 or 4.");
    }
}