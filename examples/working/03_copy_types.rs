// Fixed-size stack types like i32, bool, and f64 implement Copy,
// so assignment duplicates them and the original stays valid.
fn main() {
    let x = 5;
    let y = x;
    println!("{} {}", x, y);

    let flag = true;
    let flag2 = flag;
    println!("{} {}", flag, flag2);
}
