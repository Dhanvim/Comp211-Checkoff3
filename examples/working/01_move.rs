// Moving a String: ownership transfers from s1 to s2.
// After the move, only s2 may be used.
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // MOVE: s1 is now invalid
    println!("{}", s2);
}
