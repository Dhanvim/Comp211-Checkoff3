// A moved-from variable is only unusable until you assign it a new value.
fn main() {
    let mut a = String::from("hello");
    let b = a; // a is moved
    a = String::from("world"); // a gets a fresh value, so it is valid again
    println!("{} {}", a, b);
}
