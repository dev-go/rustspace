/*
所有权:
    (1) 每个值都有一个变量, 这个变量是该值的所有者
    (2) 每个值同时只能有一个所有者
    (3) 当所有者超出作用域(scope)时, 该值将被删除
*/

fn main() {
    {
        println!("=== >>> move");

        let s1 = String::from("abc");
        println!("s1 = {:?}", s1);
        let s2 = s1;
        println!("s2 = {:?}", s2);
        // println!("s1 = {:?}", s1); // error[E0382]: borrow of moved value: `s1`

        let t1 = (88, String::from("xyz"));
        println!("t1 = {:?}", t1);
        let t2 = t1;
        println!("t2 = {:?}", t2);
        // println!("t1 = {:?}", t1); // borrow of moved value: `t1`

        let a1 = [
            String::from("aaa"),
            String::from("bbb"),
            String::from("ccc"),
        ];
        println!("a1 = {:?}", a1);
        let a2 = a1;
        println!("a2 = {:?}", a2);
        // println!("a1 = {:?}", a1); // borrow of moved value: `a1`
    }

    {
        println!("=== >>> copy");

        let n1 = 88;
        println!("n1 = {:?}", n1);
        let n2 = n1;
        println!("n2 = {:?}", n2);
        println!("n1 = {:?}", n1);

        let t1 = (11, 99.9);
        println!("t1 = {:?}", t1);
        let t2 = t1;
        println!("t2 = {:?}", t2);
        println!("t1 = {:?}", t1);

        let a1 = [11, 22, 33];
        println!("a1 = {:?}", a1);
        let a2 = a1;
        println!("a2 = {:?}", a2);
        println!("a1 = {:?}", a1);
    }

    {
        println!("=== >>> clone");

        let s1 = String::from("hello");
        println!("s1 = {:?}", s1);
        let s2 = s1.clone();
        println!("s2 = {:?}", s2);
        println!("s1 = {:?}", s1);

        let t1 = (88, String::from("xyz"));
        println!("t1 = {:?}", t1);
        let t2 = t1.clone();
        println!("t2 = {:?}", t2);
        println!("t1 = {:?}", t1);

        let a1 = [
            String::from("aaa"),
            String::from("bbb"),
            String::from("ccc"),
        ];
        println!("a1 = {:?}", a1);
        let a2 = a1.clone();
        println!("a2 = {:?}", a2);
        println!("a1 = {:?}", a1);
    }

    {
        println!("=== >>> function");

        let n1 = 30000;
        let n2 = 31000;
        println!("n1 = {n1}, n2 = {n2}");
        let (sum, overflowed) = calc_add(n1, n2);
        println!("sum = {sum}, overflowed = {overflowed}");
        println!("n1 = {n1}, n2 = {n2}");

        let s1 = String::from("hello");
        println!("s1 = {:?}", s1);
        let s1_len = str_len(s1);
        println!("s1_len = {s1_len}");
        // println!("s1 = {:?}", s1); // borrow of moved value: `s1`

        // let s2 = String::from("hello");
        // println!("s2 = {:?}", s2);
        // let s2_len: usize;
        // (s2, s2_len) = str_len2(s2); // cannot assign twice to immutable variable `s2`

        let mut s3 = String::from("hello");
        println!("s3 = {:?}", s3);
        let s3_len: usize;
        (s3, s3_len) = str_len2(s3);
        println!("s3_len = {:?}", s3_len);
        println!("s3 = {:?}", s3);

        let s4 = String::from("hello");
        println!("s4 = {:?}", s4);
        let (s4, s4_len) = str_len2(s4);
        println!("s4_len = {:?}", s4_len);
        println!("s4 = {:?}", s4);

        let s5 = String::from("hello");
        println!("s5 = {:?}", s5);
        let (s5_len, s5) = str_len3(s5);
        println!("s5_len = {:?}", s5_len);
        println!("s5 = {:?}", s5);
    }
}

fn calc_add(a: i16, b: i16) -> (i16, bool) {
    a.overflowing_add(b)
}

fn str_len(s: String) -> usize {
    s.len()
}

fn str_len2(s: String) -> (String, usize) {
    // (s, s.len())
    // (s, s.clone().len())
    (s.clone(), s.len())
}

fn str_len3(s: String) -> (usize, String) {
    (s.len(), s)
}
