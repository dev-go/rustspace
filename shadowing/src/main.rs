fn main() {
    let a = 100;
    println!("a = {a}");

    // a = 100.1; // error[E0308]: mismatched types
    let a = 100.1;
    println!("a = {a}");
}
