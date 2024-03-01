fn main() {
    println!("*** *** Advance Unsafe *** ***");

    {
        println!("*** *** *** ***");
        let mut n = 5;
        let r1 = &n as *const i32;
        let r2 = &mut n as *mut i32;
        println!("n = {:?}, r1 = {:?}, r2 = {:?}", n, r1, r2);

        // println!("r1 = {:?}", *r1); // dereference of raw pointer is unsafe and requires unsafe function or block
        // println!("r2 = {:?}", *r2); // dereference of raw pointer is unsafe and requires unsafe function or block

        unsafe {
            println!("r1 = {:?}, *r1 = {:?}", r1, *r1);
            println!("r2 = {:?}, *r2 = {:?}", r2, *r2);
        }

        let p = 0x16fca0000_usize;
        let addr = p as *const i32;
        println!("p = {:?}, addr = {:?}", p, addr);

        // println!("addr = {:?}, *addr = {:?}", addr, *addr); // dereference of raw pointer is unsafe and requires unsafe function or block

        // unsafe {
        //     println!("addr = {:?}, *addr = {:?}", addr, *addr); // zsh: segmentation fault  cargo run
        // }
    }

    {
        println!("*** *** *** ***");
        unsafe fn dangerous() {}

        // dangerous(); // call to unsafe function is unsafe and requires unsafe function or block

        unsafe {
            dangerous();
        }
    }

    {
        println!("*** *** *** ***");

        fn split_slice(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
            let len = slice.len();
            if mid >= len {
                panic!("bad parameter: slice.len={len}, mid={mid}");
            }
            // (&mut slice[..mid], &mut slice[mid..]) // cannot borrow `*slice` as mutable more than once at a time
            unsafe {
                (
                    std::slice::from_raw_parts_mut(slice.as_mut_ptr(), mid),
                    std::slice::from_raw_parts_mut(slice.as_mut_ptr().add(mid), len - mid),
                )
            }
        }

        let v1 = &mut ([11, 22, 33, 44, 55])[..];
        let mut v2 = [11, 22, 33, 44, 55];
        let mid = 3;
        let (before1, after1) = split_slice(v1, mid);
        println!("before1: {:?}, after1: {:?}", before1, after1);
        let (before2, after2) = split_slice(&mut v2, mid);
        println!("before2: {:?}, after2: {:?}", before2, after2);
    }

    {
        println!("*** *** *** ***");
        extern "C" {
            fn abs(input: i32) -> i32;
        }

        unsafe {
            println!("abs -3 : {}", abs(-3));
        }

        #[no_mangle]
        pub extern "C" fn call_from_other() {
            println!("Call a Rust function from Rust or other programming language!");
        }

        call_from_other();
    }

    {
        println!("*** *** *** ***");
        static mut COUNTER: u32 = 0;
        fn incr_counter(incr: u32) -> u32 {
            unsafe {
                COUNTER = COUNTER + incr;
                COUNTER
            }
        }
        fn get_counter() -> u32 {
            unsafe { COUNTER }
        }

        println!("COUNTER: {:?}", get_counter());
        println!("COUNTER: {:?}", incr_counter(2));
        println!("COUNTER: {:?}", get_counter());
        println!("COUNTER: {:?}", incr_counter(5));
        println!("COUNTER: {:?}", get_counter());
    }

    {
        println!("*** *** *** ***");
        static mut COUNTER: std::sync::Mutex<Option<u32>> = std::sync::Mutex::new(Option::None);
        let join1 = std::thread::spawn(|| unsafe {
            let mut counter = COUNTER.lock().unwrap();
            *counter = Option::Some(1024u32);
        });
        let join2 = std::thread::spawn(|| unsafe {
            std::thread::sleep(std::time::Duration::from_millis(100));
            let counter = COUNTER.lock().unwrap();
            println!("counter = {:?}", counter);
        });
        let join3 = std::thread::spawn(|| unsafe {
            std::thread::sleep(std::time::Duration::from_millis(200));
            let mut counter = COUNTER.lock().unwrap();
            *counter = Option::Some(2048u32);
        });
        let join4 = std::thread::spawn(|| unsafe {
            std::thread::sleep(std::time::Duration::from_millis(300));
            let counter = COUNTER.lock().unwrap();
            println!("counter = {:?}", counter);
        });
        join1.join();
        join2.join();
        join3.join();
        join4.join();
        unsafe {
            println!("counter = {:?}", COUNTER.lock().unwrap());
        }
    }

    {
        println!("*** *** *** ***");
        unsafe trait Foo {}

        // impl Foo for i32 {} // the trait `Foo` requires an `unsafe impl` declaration

        unsafe impl Foo for i32 {}
    }
}
