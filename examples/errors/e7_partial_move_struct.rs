// EXPECTED ERROR: E0382 borrow of moved value: `p.name`
// Moving one field leaves the struct partially invalid. p.age is still fine (Copy).
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let p = Person { name: String::from("Ana"), age: 20 };
    let n = p.name;
    println!("{}", p.name);
    println!("{} {}", n, p.age);
}
