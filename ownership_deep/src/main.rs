#[derive(Debug)]
struct User {
    id: u64,
}

// 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
//      不可变引用（shared reference）实现了 Copy trait，不会发生所有权转移
//      可变引用（mutable reference）未实现 Copy trait，会发生所有权转移

fn main() {
    {
        let s1 = String::from("abc");
        println!("s1 = {:?}", s1);
        let s2 = s1;
        println!("s2 = {:?}", s2);
        // println!("s1 = {:?}", s1); // borrow of moved value: `s1`
    }

    {
        let s1 = &String::from("opq");
        println!("s1 = {:?}", s1);
        let s2 = s1;
        println!("s2 = {:?}", s2);
        println!("s1 = {:?}", s1);
    }

    {
        let s1 = &mut String::from("xyz");
        println!("s1 = {:?}", s1);
        let s2 = s1;
        println!("s2 = {:?}", s2);
        // println!("s1 = {:?}", s1); // borrow of moved value: `s1`
    }

    {
        let mut s1 = &String::from("abc");
        println!("s1 = {:?}", s1);
        // s1 = &String::from("xyz"); // temporary value dropped while borrowed
        println!("s1 = {:?}", s1);
    }

    {
        let u1 = User { id: 1001 };
        println!("u1 = {:?}", u1);
        let u2 = u1;
        println!("u2 = {:?}", u2);
        // println!("u1 = {:?}", u1); // borrow of moved value: `u1`
    }

    {
        let mut n = 100;
        n += 1;
        let p1 = &n;
        println!("p1 = {:?}", p1);
        let p2 = p1;
        println!("p2 = {:?}", p2);
        println!("n = {:?}, p1 = {:?}, p2 = {:?}", n, p1, p2);
    }

    {
        let mut m = 100;
        m += 2;
        let p1 = &mut m;
        println!("p1 = {:?}", p1);
        let p2 = p1;
        // println!("m = {:?}", m); // cannot borrow `m` as immutable because it is also borrowed as mutable
        println!("p2 = {:?}", p2);
        // println!("p1 = {:?}", p1); // borrow of moved value: `p1`
    }
}
