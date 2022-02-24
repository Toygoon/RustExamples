fn main() {
    let x = 5;

    // Shadowed
    let x = x + 1;

    // Shadowed
    let x = x * 2;

    println!("x = {}", x);
}