fn main() {
    let a: i8 = 100;
    let b: i8 = 88;
    println!("a = {a}, b = {b}");

    let (c, overflowed) = a.overflowing_add(b);
    println!("c = {c}, overflowed = {overflowed}");

    let d = 0;
    let e = a.checked_div(d);
    println!("e = {:#?}", e);
    let f: i8;
    match e {
        Some(n) => f = n,
        None => {
            f = 0;
            println!("can not exec div");
        }
    };
    println!("f = {f}");

    let g: i8 = 8;
    let h = a.checked_div(g);
    println!("h = {:#?}", h);
    let i: i8;
    match h {
        Some(n) => i = n,
        None => {
            println!("can not exec dive");
            i = 0;
        }
    };
    println!("i = {i}");
}
