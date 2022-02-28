fn main() {
    println!("Hello!");

    another_function();
}

fn another_function(x: i32, y: i32) {
    println!("x의 값 : {}", x);
    println!("y의 값 : {}", y);
}