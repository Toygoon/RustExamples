fn main() {
    // Introducing local variables in main function
    let s = String::from("hello");

    // Variable s is moving into the takes_ownership function
    // And moving that, variable s is unavailable from main
    // Similar with Call by Reference and delete s;
    takes_ownership(s);

    // Introducing local variables in main function
    let x = 5;

    // Variable x is moving into the makes_copy function
    // But unlike the past conditions, i32 type is performing automated deep-copy
    // So, variable x is still accessable in main function
    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
    // some_string drops, deallocating from the memory
}

fn makes_copy(some_integer: i32) {
    // some_interger is copied
    println!("{}", some_integer);
}