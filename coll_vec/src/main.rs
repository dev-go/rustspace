fn main() {
    let mut v1 = vec![11, 22, 33];
    println!("v1 = {:?}", v1);
    v1.push(44);
    println!("v1 = {:?}", v1);

    let mut v2 = Vec::new();
    v2.push(String::from("aaa"));
    v2.push(String::from("bbb"));
    v2.push(String::from("ccc"));
    println!("v2 = {:?}", v2);
    // let e21 = v2[1]; // cannot move out of index of `Vec<String>`
    // println!("e21 = {:?}", e21);
    let e21 = &v2[1];
    println!("e21 = {:?}", e21);
    // e21.push_str("..."); // cannot borrow `*e21` as mutable, as it is behind a `&` reference

    // v2.push(String::from("ddd")); // cannot borrow `v2` as mutable because it is also borrowed as immutable
    println!("e21 = {:?}", e21);
    v2.push(String::from("ddd"));
    println!("v2 = {:?}", v2);

    let e22 = &mut v2[2];
    println!("e22 = {:?}", e22);
    e22.push_str("...");
    println!("e22 = {:?}", e22);
    println!("v2 = {:?}", v2);

    {
        println!("=== >>> get");
        // let e28 = &v2[8]; // index out of bounds: the len is 4 but the index is 8
        // println!("e28 = {:?}", e28);
        let e28 = v2.get(8);
        println!("e28 = {:?}", e28);

        let e23 = v2.get_mut(3);
        println!("e23 = {:?}", e23);
        if let Some(v) = e23 {
            v.push_str("...");
        }
        // println!("e23 = {:?}", e23); // borrow of partially moved value: `e23`
        println!("v2 = {:?}", v2);
        println!("v2_len = {:?}", v2.len());
    }

    {
        for e in v1 {
            println!("e = {:?}", e);
        }
        // println!("v1 = {:?}", v1); // borrow of moved value: `v1`

        for e in &mut v2 {
            e.push_str(" ---");
        }
        println!("v2 = {:?}", v2);

        for (i, e) in v2.iter_mut().enumerate() {
            e.push('*');
            println!("i = {i}, e = {e}");
        }
        println!("v2 = {:?}", v2);
    }
}
