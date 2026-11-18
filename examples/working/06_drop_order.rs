// Shows when values are dropped (freed). Types that implement Drop
// let us print a message at the moment the value is freed.
struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("dropping {}", self.0);
    }
}

fn take(n: Noisy) {
    println!("take() received {}", n.0);
} // n is dropped here

fn main() {
    let a = Noisy("a");
    let b = Noisy("b");
    take(a); // a is moved into take() and dropped when take() ends
    println!("back in main");
    let _ = &b; // b is dropped at the closing brace of main
}
