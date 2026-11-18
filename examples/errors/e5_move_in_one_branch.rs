// EXPECTED ERROR: E0382 borrow of moved value: `s`
// The compiler assumes the worst case: some path might have moved s.
fn consume(s: String) {
    println!("{}", s);
}

fn main() {
    let s = String::from("hi");
    let some_condition = std::env::args().count() > 1;
    if some_condition {
        consume(s);
    }
    println!("{}", s);
}
