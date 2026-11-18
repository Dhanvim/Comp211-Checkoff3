// EXPECTED ERROR: E0382 borrow of moved value: `a`
// C equivalent: use-after-free (the function already freed the buffer).
fn consume(s: String) {
    println!("{}", s);
}

fn main() {
    let a = String::from("hello");
    consume(a);
    println!("{}", a);
}
