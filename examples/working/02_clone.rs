// .clone() makes a deep copy (new heap allocation), so both variables stay valid.
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{} {}", s1, s2);
}
