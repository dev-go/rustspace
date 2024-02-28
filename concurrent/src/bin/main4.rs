fn main() {
    println!("*** *** Concurrent *** ***");

    let locker = std::sync::Mutex::new(String::from("abc"));
    println!("locker = {:?}", locker);

    let mut a = locker.lock().unwrap();
    println!("a = {:?}", a);
    (*a).push_str("...");
    println!("a = {:?}", a);

    println!("locker = {:?}", locker);

    // // 死锁
    // let mut b = locker.lock().unwrap();

    drop(a);
    println!("locker = {:?}", locker);

    {
        let mut b = locker.lock().unwrap();
        (*b).push_str("xyz");
        println!("locker = {:?}", locker);
    }
    println!("locker = {:?}", locker);
}
