use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("System generated value : {}", secret_number);
    
    println!("Let's guess a number.");

    loop {
        println!("Input a number.");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Couldn't read a number.");

        let guess: i32 = guess.trim().parse().expect("Invalid input.");
        println!("Input value : {}", guess);
            
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Less number"),
            Ordering::Greater => println!("Greater number"),
            Ordering::Equal => {
                println!("Correct!");
                break;}
        }
    }
}
