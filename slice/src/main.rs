fn main() {
    {
        println!("=== >>> slice immut");

        let a1 = [11, 22, 33, 44];
        println!("a1 = {:#?}", a1);
        let a2 = a1.as_slice();
        println!("a2 = {:#?}", a2);
        let a3 = &a1[1..3];
        println!("a3 = {:#?}", a3);
        let a4 = &a1[..3];
        println!("a4 = {:#?}", a4);
        let a5 = &a1[1..];
        println!("a5 = {:#?}", a5);
        let a6 = &a1[..];
        println!("a6 = {:#?}", a6);
        let a7 = &a1;
        println!("a7 = {:#?}", a7);
    }

    {
        println!("=== >>> slice mut");

        let mut a1 = [11, 22, 33, 44];
        println!("a1 = {:#?}", a1);
        let a2 = &mut a1[1..];
        a2[0] = 100;
        println!("a2 = {:#?}", a2);
        // println!("a1 = {:#?}", a1); // cannot borrow `a1` as immutable because it is also borrowed as mutable
        println!("a2 = {:#?}", a2);
        println!("a1 = {:#?}", a1);
    }

    {
        println!("=== >>> string");

        let s0: &'static str = "hello world";
        let mut s1 = String::from(s0);
        println!("s1 = {:#?}", s1);
        let s2 = first_word(&s1);
        println!("s2 = {:#?}", s2);
        // s1.clear(); // cannot borrow `s1` as mutable because it is also borrowed as immutable
        println!("s2 = {:#?}", s2);
        s1.clear();
    }
}

fn first_word(s: &String) -> &str {
    for (i, e) in s.as_bytes().iter().enumerate() {
        if *e == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}
