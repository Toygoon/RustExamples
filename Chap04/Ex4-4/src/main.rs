fn main() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    
    // Return some_string
    some_string
}

// takes_and_gives_back function just returns the String from the String that parameter has given
fn takes_and_gives_back(a_string: String) -> String {
    // a_string is moved to here
    
    a_string
}
