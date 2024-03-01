fn main() {
    println!("*** *** Advance Func *** ***");

    {
        println!("*** *** *** ***");

        fn add_five(p: i32) -> i32 {
            p + 5
        }

        fn double_exec(f: fn(i32) -> i32, p: i32) -> i32 {
            f(p) + f(p)
        }

        let p = 10;
        let r1 = double_exec(add_five, p);
        println!("r1 = {:?}", r1);
        let r2 = double_exec(|arg| arg + 10, p);
        println!("r2 = {:?}", r2);
        // let r3 = double_exec(|arg| arg + p, p); // mismatched types
        // println!("r3 = {:?}", r3);
    }

    {
        println!("*** *** *** ***");
        let v1 = vec![11, 22, 33, 44];
        println!("v1 = {:?}", v1);
        let v2: Vec<String> = v1.iter().map(|e| e.to_string()).collect();
        println!("v2 = {:?}", v2);
        let v3: Vec<String> = v1.iter().map(ToString::to_string).collect();
        println!("v3 = {:?}", v3);
    }

    {
        println!("*** *** *** ***");

        #[derive(Debug)]
        enum Status {
            Value(i32),
            Stop,
        }

        let v: Vec<Status> = (0..=5).map(Status::Value).collect();
        println!("v = {:?}", v);
    }

    {
        println!("*** *** *** ***");

        fn demo1() -> fn(i32) -> i32 {
            fn f(p: i32) -> i32 {
                p + 10
            }
            f
        }

        fn demo2() -> fn(i32) -> i32 {
            |p| p + 6
        }

        // fn demo3() -> fn(i32) -> i32 {
        //     let x = 10;
        //     |p| p + x // mismatched types
        // }

        // fn demo4() -> Fn(i32) -> i32 { // trait objects must include the `dyn` keyword
        //     let x = 10;
        //     |p| p + x
        // }

        // fn demo5() -> dyn Fn(i32) -> i32 { // return type cannot have an unboxed trait object
        //     let x = 10;
        //     |p| p + x
        // }

        // fn demo5() -> &dyn Fn(i32) -> i32 { // missing lifetime specifier // mismatched types
        //     let x = 10;
        //     |p| p + x
        // }

        // fn demo6() -> &'static dyn Fn(i32) -> i32 { // mismatched types
        //     let x = 10;
        //     |p| p + x
        // }

        // fn demo7() -> std::cell::RefCell<dyn Fn(i32) -> i32> { // the size for values of type `(dyn Fn(i32) -> i32 + 'static)` cannot be known at compilation time
        //     let x = 10;
        //     std::cell::RefCell::new(|p| p + x)
        // }

        // fn demo8() -> Box<dyn Fn(i32) -> i32> {
        //     let x = 10;
        //     Box::new(|p| p + x) // closure may outlive the current function, but it borrows `x`, which is owned by the current function
        // }

        fn demo9() -> Box<dyn Fn(i32) -> i32> {
            let x = 10;
            Box::new(move |p| p + x)
        }
    }
}
