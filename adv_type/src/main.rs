fn main() {
    println!("*** *** Advance Type *** ***");

    {
        println!("*** *** type *** ***");

        type Meter = i32;
        type KMeter = i32;
        let a = 11;
        let b: Meter = 22;
        let c: KMeter = 33;
        let d = a + b + c;
        println!("d = {d}");
        let e = a + b + c * 1000;
        println!("e = {e}");
    }

    {
        println!("*** *** type *** ***");
        fn with_long_param(f: Box<dyn Fn() + Send + 'static>) {
            f()
        }
        fn with_long_return() -> Box<dyn Fn() + Send + 'static> {
            Box::new(|| println!("with long return"))
        }
        let f: Box<dyn Fn() + Send + 'static> = Box::new(|| {
            println!("f");
        });

        type FuncBox = Box<dyn Fn() + Send + 'static>;

        fn with_long_param2(f: FuncBox) {
            f()
        }
        fn with_long_return2() -> FuncBox {
            Box::new(|| println!("with long return"))
        }

        let f2: FuncBox = Box::new(|| {
            println!("f");
        });
    }

    {
        println!("*** *** type *** ***");

        trait Write {
            fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error>;
            fn flush(&mut self) -> Result<(), std::io::Error>;
        }

        type MyResult<T> = std::io::Result<T>;

        trait Write2 {
            fn write(&mut self, buf: &[u8]) -> MyResult<usize>;
            fn flush(&mut self) -> MyResult<()>;
        }

        type MyResultWithoutError<T> = Result<T, std::io::Error>;

        trait Write3 {
            fn write(&mut self, buf: &[u8]) -> MyResultWithoutError<usize>;
            fn flush(&mut self) -> MyResultWithoutError<()>;
        }
    }

    {
        println!("*** *** ! Never *** ***");

        fn foo() -> u32 {
            println!("foo: ---");
            let x = {
                println!("foo: +++");
                return 123;
            };
            println!("foo: ===");
        }
        println!("foo: {}", foo());

        let mut count = 0;
        loop {
            count += 1;
            if count > 3 {
                break;
            }
            let input = String::from("11.2");
            let input2: i32 = match input.trim().parse() {
                Result::Ok(n) => n,
                Result::Err(e) => {
                    println!("parse input failed: {e}");
                    continue;
                }
            };
            println!("input: {:?}, input2: {:?}", input, input2);
        }
    }

    {
        println!("*** *** ?Sized *** ***");

        // fn make_array(len: i32) {
        //     let a = [0; len]; // attempt to use a non-constant value in a constant
        // }

        // let b: str = "hello"; // mismatched types // the size for values of type `str` cannot be known at compilation time
        let c: &str = "hello";

        trait MyThing {};

        fn foobar_1(thing: &dyn MyThing) {}
        fn foobar_2(thing: Box<dyn MyThing>) {}
        fn foobar_3(thing: std::rc::Rc<dyn MyThing>) {}
        // fn foobar_4(thing: MyThing) {} // trait objects must include the `dyn` keyword
        // fn foobar_5(thing: &MyThing) {} // trait objects must include the `dyn` keyword
        // fn foobar_6(thing: Box<MyThing>) {} // trait objects must include the `dyn` keyword

        // let s1: Box<str> = Box::new("123" as str); // cast to unsized type: `&'static str` as `str`
        let s2: Box<str> = "xyz".into();
        println!("s2 = {:?}", s2);

        fn gen1<T>(t: T) {}
        fn gen2<T: Sized>(t: T) {}
        // fn gen3<T: ?Sized>(t: T) {} // the size for values of type `T` cannot be known at compilation time
        fn gen4<T: ?Sized>(t: &T) {}
    }
}
