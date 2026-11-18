// EXPECTED ERROR: E0382 borrow of moved value: `a`
// C equivalent: free(p); printf("%s", p);
fn main() {
    let a = String::from("hello");
    drop(a);
    println!("{}", a);
}
