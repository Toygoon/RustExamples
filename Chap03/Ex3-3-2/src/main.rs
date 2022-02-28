fn main() {
    let x = 5;

    let y = {
        let x = 3;
        // Expression statement doesn't have semicolons at the end of the line
        x + 1
    };

    // y = 4, because y returns x
    println!("y = {}", y);
}