// EXPECTED ERROR: E0382 use of moved value: `s`
// C equivalent: calling free(s) on every loop iteration.
fn consume(s: String) {
    println!("{}", s);
}

fn main() {
    let s = String::from("hi");
    for _ in 0..3 {
        consume(s);
    }
}
