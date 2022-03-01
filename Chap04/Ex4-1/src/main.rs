fn main() {
    let s1 = String::from("hello");
    // Deep copy
    let s2 = s1.clone();

    let x = 5;
    // Deep copy is unneccesary for normal i32 types
    let y = x;

    println!("s1 = {}, s2 = {}", s1, s2);
    println!("x = {}, y = {}", x, y);
}