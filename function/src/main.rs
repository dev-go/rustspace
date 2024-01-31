fn main() {
    let a: i64 = 88;
    let b: i64 = 1000;
    let (c, d) = add(a, b);
    println!("a = {a}, b = {b}, c = {c}, d = {d}");

    let name = String::from("liu");
    println!("name = {:?}", name);
    let greeting = greet(name);
    println!("greeting: {:?}", greeting);
    // println!("name = {:?}", name); // error[E0382]: borrow of moved value: `name`
}

fn add(x: i64, y: i64) -> (i64, bool) {
    x.overflowing_add(y)
}

fn greet(name: String) -> String {
    let mut s = String::from("Hello, ");
    s.push_str(name.as_str());
    s.push('!');
    s
}
