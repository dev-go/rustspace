fn main() {
    let t1 = ();
    println!("t1 = {:#?}", t1);

    let t2 = (66,);
    println!("t2 = {:#?}", t2);

    let t3 = (88, String::from("abc"));
    println!("t3 = {:#?}", t3);

    let mut t4 = (66, String::from("xyz"));
    println!("t4 = {:#?}", t4);

    {
        println!("=== === >>> >>>");
        println!("t2 = {:#?}", t2);
        let v = (88);
        println!("v = {:#?}", v);

        let (a) = t2;
        println!("a = {:#?}", a);

        let (b,) = t2;
        println!("b = {:#?}", b);

        println!("t2.0 = {:#?}", t2.0);

        // t2.0 += 1; // error[E0594]: cannot assign to `t2.0`, as `t2` is not declared as mutable
    }

    {
        println!("=== === >>> >>>");

        // let (a, b) = &mut t3; // error[E0596]: cannot borrow `t3` as mutable, as it is not declared as mutable
        let (a, b) = &t3;
        println!("t3 = {:#?}", t3);
        println!("a = {a}, b = {b}");
        println!("t3 = {:#?}", t3);

        let (c, ref d) = t3;
        println!("c = {c}, d = {d}");
        println!("t3 = {:#?}", t3);

        let (mut e, mut f) = t3;
        println!("e = {e}, f = {f}");
        e += 100;
        f.push_str("...");
        println!("e = {e}, f = {f}");
        // println!("t3 = {:#?}", t3); // error[E0382]: borrow of partially moved value: `t3`
    }

    {
        println!("=== === >>> >>>");
        println!("t4 = {:#?}", t4);
        let (a, b) = &mut t4;
        println!("a = {:#?}, b = {:#?}", a, b);
        *a += 200;
        b.push_str("...");
        println!("a = {:#?}, b = {:#?}", a, b);
        println!("t4 = {:#?}", t4);

        let (ref mut c, ref mut d) = t4;
        println!("c = {:#?}, d = {:#?}", c, d);
        *c += 1000;
        d.push_str("---");
        println!("c = {:#?}, d = {:#?}", c, d);
        println!("t4 = {:#?}", t4);

        t4.0 += 1000;
        t4.1.push_str("xyz");
        println!("t4 = {:#?}", t4);

        let (ref e, ref f) = t4;
        println!("e = {e}, f = {f}");
        // *e += 100; // error[E0594]: cannot assign to `*e`, which is behind a `&` reference
    }

    {
        println!("=== === >>> >>>");

        let n1 = 666;
        let s1 = String::from("xyz");
        println!("n1 = {n1}, s1 = {:#?}", s1);
        let t10 = (n1, s1);
        println!("t10 = {:?}", t10);
        println!("t10 = {:#?}", t10);
        // println!("n1 = {n1}, s1 = {:#?}", s1); // error[E0382]: borrow of moved value: `s1`
    }

    {
        println!("=== === >>> >>>");

        let mut t11 = (66, String::from("abc"));
        let mut t12 = (66, String::from("abc"));
        let mut t13 = (66, String::from("abc"));
        let mut t14 = (66, 88);

        let (a, b) = t11;
        // let (a, b) = &t11; // error[E0382]: borrow of partially moved value: `t11`
        let (a, b) = &t12;
        let (a, b) = &mut t12;
        b.push_str(".");
        let (mut a, mut b) = t12;
        b.push_str(".");
        // let (ref a, ref b) = t12; // error[E0382]: borrow of moved value: `t12.1`
        let (ref a, ref b) = t13;
        let (ref mut a, ref mut b) = t13;
        // let (mut a, mut b) = &mut t13; // error[E0507]: cannot move out of a mutable reference
        let (mut a, mut b) = t14;
        let (mut a, mut b) = &mut t14;
    }
}
