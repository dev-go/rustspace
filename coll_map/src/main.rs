fn main() {
    {
        println!("=== >>> HashMap");
        let mut m1 = std::collections::HashMap::new();
        m1.insert(String::from("a"), String::from("aaa"));
        m1.insert(String::from("c"), String::from("ccc"));
        m1.insert(String::from("x"), String::from("xxx"));
        println!("m1 = {:?}", m1);
        let ex = m1.get(&String::from("x"));
        println!("ex = {:?}", ex);
        if let Option::Some(e) = ex {
            println!("ex value is {:?}", e);
        }
        println!("ex = {:?}", ex);

        let ec: Option<&mut String> = m1.get_mut(&String::from("c"));
        println!("ec = {:?}", ec);
        if let Option::Some(e) = ec {
            println!("ec value is {:?}", e);
        }
        // println!("ec = {:?}", ec); // borrow of partially moved value: `ec`

        let ea = m1.get_mut(&String::from("a"));
        println!("ea = {:?}", ea);
        if let Some(ref e) = ea {
            println!("ea value is {:?}", e);
        }
        println!("ea = {:?}", ea);

        println!("m1 = {:?}", m1);

        for (k, v) in &mut m1 {
            println!("k = {:?}, v = {:?}", k, v);
            let v2 = v;
            println!("v2 = {:?}", v2);
            // println!("v = {:?}", v); // borrow of moved value: `v`
        }

        m1.entry(String::from("b")).or_insert(String::from("bbb"));
        m1.entry(String::from("c")).or_insert(String::from("***"));
        println!("m1 = {:?}", m1);

        let mut counter = std::collections::HashMap::new();
        let text = String::from("hello world hello c hello go hello rust hello c");
        for word in text.split_whitespace() {
            let count = counter.entry(word).or_insert(0);
            *count += 1;
        }
        println!("counter = {:?}", counter);
    }
}
