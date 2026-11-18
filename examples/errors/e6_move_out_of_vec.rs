// EXPECTED ERROR: E0507 cannot move out of index of `Vec<String>`
// If `first` owned v[0], the Vec would free the same buffer again (double free).
fn main() {
    let v = vec![String::from("a"), String::from("b")];
    let first = v[0];
    println!("{}", first);
}
