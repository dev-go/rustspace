fn main() {
    {
        println!("=== >>> 声明与初始化");

        let a1 = [11, 22, 33, 44];
        println!("a1 = {:?}", a1);
        let a2 = [0; 4];
        println!("a2 = {:?}", a2);
        let a3 = [4; 0];
        println!("a3 = {:?}", a3);
        // let a4: [i32; 4] = [111, 222, 333]; // error[E0308]: mismatched types
        // println!("a4 = {:?}", a4);
        let a5: [i32; 4] = [111, 222, 333, 444];
        println!("a5 = {:?}", a5);
    }

    {
        println!("=== >>> 可变性与索引");

        let a1: [i32; 4] = [111, 222, 333, 444];
        println!("a1 = {:?}", a1);
        let e11 = a1[1];
        println!("e11 = {:?}", e11);
        // a1[0] += 1; // cannot assign to `a1[_]`, as `a1` is not declared as mutable
        // println!("a1 = {:?}", a1);

        let mut a2 = [111, 222, 333, 444];
        println!("a2 = {:?}", a2);
        let e21 = a2[1];
        println!("e21 = {:?}", e21);
        a2[0] += 1;
        println!("a2 = {:?}", a2);

        let a3 = [
            String::from("aaa"),
            String::from("bbb"),
            String::from("ccc"),
        ];
        println!("a3 = {:?}", a3);
        // let e31 = a3[1]; // error[E0508]: cannot move out of type `[String; 3]`, a non-copy array
        // println!("e31 = {:?}", e31);
        let e31 = &a3[1];
        println!("e31 = {:?}", e31);
        let ref e32 = a3[2];
        println!("e32 = {:?}", e32);
        // a3[0].push_str("..."); // error[E0596]: cannot borrow `a3[_]` as mutable, as `a3` is not declared as mutable
        println!("a3 = {:?}", a3);

        let mut a4 = [
            String::from("xxx"),
            String::from("yyy"),
            String::from("zzz"),
            String::from("###"),
            String::from("***"),
        ];
        println!("a4 = {:?}", a4);
        // let e41 = a4[1]; // error[E0508]: cannot move out of type `[String; 3]`, a non-copy array
        // println!("e41 = {:?}", e41);
        let e41 = &a4[1];
        println!("e41 = {:?}", e41);
        let ref e42 = a4[2];
        println!("e42 = {:?}", e42);
        // e42.push_str("..."); // error[E0596]: cannot borrow `*e42` as mutable, as it is behind a `&` reference
        let ref mut e43 = a4[3];
        println!("e43 = {:?}", e43);
        e43.push_str("...");
        let mut e44 = &mut a4[4];
        println!("e44 = {:?}", e44);
        e44.push_str("...");
        println!("a4 = {:?}", a4);
        let mut s = String::from("xyz");
        e44 = &mut s;
        println!("e44 = {:?}", e44);
        a4[0].push_str("...");
        println!("a4 = {:?}", a4);
    }

    {
        println!("=== >>> 遍历");

        let a1: [i32; 4] = [111, 222, 333, 444];
        println!("a1 = {:?}", a1);
        for e in a1 {
            println!("a1.elem = {e}");
        }
        for (i, e) in a1.iter().enumerate() {
            println!("a1[{i}] = {e}");
        }
        println!("a1 = {:?}", a1);

        let a2 = [
            String::from("xxx"),
            String::from("yyy"),
            String::from("zzz"),
            String::from("###"),
            String::from("***"),
        ];
        println!("a2 = {:?}", a2);
        for e in a2 {
            println!("a2.elem = {e}");
        }
        // println!("a2 = {:?}", a2); // error[E0382]: borrow of moved value: `a2`

        let a3 = [
            String::from("xxx"),
            String::from("yyy"),
            String::from("zzz"),
            String::from("###"),
            String::from("***"),
        ];
        println!("a3 = {:?}", a3);
        for ref e in a3 {
            println!("a3.elem = {e}");
        }
        // println!("a3 = {:?}", a3); // error[E0382]: borrow of moved value: `a3`

        let a4 = [
            String::from("xxx"),
            String::from("yyy"),
            String::from("zzz"),
            String::from("###"),
            String::from("***"),
        ];
        println!("a4 = {:?}", a4);
        for e in &a4 {
            println!("a4.elem = {e}");
        }
        println!("a4 = {:?}", a4);
    }
}
