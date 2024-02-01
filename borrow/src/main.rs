fn main() {
    {
        println!("=== >>> borrow");

        let n1 = 100;
        // let p0 = &mut n1; // cannot borrow `n1` as mutable, as it is not declared as mutable
        let p1 = &n1;
        let p2 = &n1;
        println!("n1 = {:#?}, p1 = {:#?}, p2 = {:#?}", n1, p1, p2);

        let mut n2 = 88;
        let p3 = &mut n2;
        *p3 += 1;
        let p4 = &mut n2;
        *p4 += 1;
        let p5 = &mut n2;
        *p5 += 66;
        // println!("p5 = {:#?}, n2 = {:#?}", p5, n2); // cannot borrow `n2` as immutable because it is also borrowed as mutable
        println!("p5 = {:#?}", p5);
        // println!("n2 = {:#?}", n2); // cannot borrow `n2` as immutable because it is also borrowed as mutable
        println!("p5 = {:#?}", p5);
        println!("n2 = {:#?}", n2);
    }

    {
        println!("=== >>> borrow");

        let s1 = String::from("abc");
        // let p0 = &mut s1; // cannot borrow `s1` as mutable, as it is not declared as mutable
        let p1 = &s1;
        let p2 = &s1;
        println!("s1 = {:#?}, p1 = {:#?}, p2 = {:#?}", s1, p1, p2);

        let mut s2 = String::from("abc");
        let p3 = &mut s2;
        p3.push('*');
        let p4 = &mut s2;
        p4.push_str("...");
        let p5 = &mut s2;
        p5.push_str("^^^");
        // println!("p5 = {:#?}, s2 = {:#?}", p5, s2); // cannot borrow `s2` as immutable because it is also borrowed as mutable
        println!("p5 = {:#?}", p5);
        // println!("s2 = {:#?}", s2); // cannot borrow `s2` as immutable because it is also borrowed as mutable
        println!("p5 = {:#?}", p5);
        println!("s2 = {:#?}", s2);
    }

    {
        println!("=== >>> function");

        let mut s1 = String::from("hello");
        println!("s1 = {:#?}", s1);
        let s1_len = str_len(&s1);
        println!("s1_len = {:#?}", s1_len);
        let s1_len = greet(&mut s1);
        println!("s1_len = {:#?}", s1_len);
        println!("s1 = {:#?}", s1);
    }
}

fn str_len(s: &String) -> usize {
    s.len()
}

fn greet(name: &mut String) -> usize {
    name.push_str("...");
    name.len()
}

// fn dangle() -> &String { // error[E0106]: missing lifetime specifier
//     let s = String::from("ok");
//     &s
// }
