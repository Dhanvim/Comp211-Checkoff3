// Passing a String into a function moves it. The function frees it at the end.
// Fix: have the function hand ownership back by returning the value.
fn consume(s: String) {
    println!("consume got: {}", s);
} // s dropped here

fn consume_and_return(s: String) -> String {
    println!("consume_and_return got: {}", s);
    s // ownership goes back to the caller
}

fn main() {
    let a = String::from("hello");
    consume(a); // a is moved; do not use it after this line

    let b = String::from("world");
    let b = consume_and_return(b); // ownership comes back
    println!("main still has: {}", b);
}
