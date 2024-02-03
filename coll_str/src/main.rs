fn main() {
    {
        println!("=== >>> String");
        let s1 = String::from("abc");
        println!("s1 = {:?}", s1);

        let n1 = 10086;
        let s2 = n1.to_string();
        println!("s2 = {:?}", s2);

        let b1 = true;
        let s3 = b1.to_string();
        println!("s3 = {:?}", s3);

        let bytes = vec![70_u8, 71_u8, 72_u8];
        let s4 = String::from_utf8(bytes);
        println!("s4 = {:?}", s4);
        // if let Ok(&s) = s4 { // mismatched types
        //     println!("s4 value is {:?}", s);
        // }
        if let Ok(ref s) = s4 {
            println!("s4 value is {:?}", s);
        }
        println!("s4 = {:?}", s4);
        if let Ok(s) = s4 {
            println!("s4 value is {:?}", s);
        }
        // println!("s4 = {:?}", s4); // borrow of partially moved value: `s4`

        let a = 100;
        let b = 3.14;
        let c = "hello";
        let s5 = format!("a = {a}, b = {b}, c = {c}");
        println!("s5 = {:?}", s5);
    }

    {
        println!("=== >>> String");
        let mut s1 = String::from("hello world");
        println!("s1 = {:?}", s1);
        // let e0 = s1[0]; // the type `String` cannot be indexed by `{integer}`
        // let e0 = &s1[0]; // the type `String` cannot be indexed by `{integer}`
        s1.push_str("...");
        println!("s1 = {:?}", s1);
        let e3 = s1.remove(3);
        println!("e3 = {:?}", e3);
        println!("s1 = {:?}", s1);

        let s2 = "rust";
        s1 = s1 + s2;
        println!("s1 = {:?}", s1);
    }

    {
        println!("=== >>> String");
        let s1 = String::from("abc");
        let s2 = String::from("中国");
        let s3 = String::from("😊😂");
        println!("s1: {:?}, len: {:?}", s1, s1.len());
        println!("s2: {:?}, len: {:?}", s2, s2.len());
        println!("s3: {:?}, len: {:?}", s3, s3.len());

        // for e in &s1 { // `&String` is not an iterator
        //     println!("e = {e}");
        // }

        for e in s1.chars() {
            println!("s1: e = {:?}", e);
        }

        for e in s1.as_bytes() {
            println!("s1: e = {:?}", e);
        }

        println!("s1 = {:?}", s1);

        for e in s2.chars() {
            println!("s2: e = {:?}", e);
        }
        for e in s2.as_bytes() {
            println!("s2: e = {:?}", e);
        }

        println!("s2 = {:?}", s2);

        let s3 = &s1[..2];
        println!("s3 = {:?}", s3);
        println!("s1 = {:?}", s1);
        println!("s3 = {:?}", s3);

        let s4 = &s2[..3];
        println!("s4 = {:?}", s4);

        // let s5 = &s2[..2]; // byte index 2 is not a char boundary; it is inside '中' (bytes 0..3) of `中国`
        // println!("s5 = {:?}", s5);
    }
}
