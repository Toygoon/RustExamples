/* fn main() {
    let s1 = String::from("Hello");

    let (s2, len) = calculate_length(s1);

    println!("Length of '{}' is {}", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
*/

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("Length of '{}' is {}", s1, len);
}

// & operator : Variables can be read, but not be granted ownership. It's a reference
fn calculate_length(s: &String) -> usize {
    s.len()
    // Variable s is dropping of the range, but nothing happens becuase there's not any ownership
}