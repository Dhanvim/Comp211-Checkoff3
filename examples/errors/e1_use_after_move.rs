// EXPECTED ERROR: E0382 borrow of moved value: `a`
// C equivalent: using a pointer after its buffer's owner changed.
fn main() {
    let a = String::from("hello");
    let b = a;
    println!("{}", a);
    println!("{}", b);
}
