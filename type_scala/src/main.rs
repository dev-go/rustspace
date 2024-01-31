fn main() {
    let a: i8 = 100;
    println!("a = {a}");
    let b: i16 = 10000;
    println!("b = {b}");
    let c: i32 = 1_0000_0000;
    println!("c = {c}");
    let d: i64 = 100_0000_0000;
    println!("d = {d}");

    // let e : u8 = 256; // error: literal out of range for `u8`
    let e: u8 = 255;
    println!("e = {e}");
    let f: u16 = 65535;
    println!("f = {f}");
    let g: u32 = 40_0000_0000;
    println!("g = {g}");
    let h: u64 = 100_0000_0000;
    println!("h = {h}");

    let i: f32 = 3.14;
    println!("i = {i}");
    let j: f64 = 3.1415925;
    println!("j = {j}");

    let k: char = 'a';
    println!("k = {:?}", k);
    let l: char = '😚';
    println!("l = {:#?}", l);

    let m: &str = "hello";
    println!("m = {:#?}", m);

    let mut n: bool = false;
    println!("n = {n}");
    n = true;
    println!("n = {n}");

    let o: isize = 20_0000_0000;
    println!("o = {o}");
    let p: usize = 100_0000_0000;
    println!("p = {p}");
}
