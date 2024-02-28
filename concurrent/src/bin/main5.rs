fn main() {
    println!("*** *** Concurrent *** ***");

    let locker = std::sync::Arc::new(std::sync::Mutex::new(String::from("abc  ")));
    println!("locker = {:?}", locker);

    let mut handles = vec![];
    for i in 0..10 {
        let locker_clone = std::sync::Arc::clone(&locker);
        handles.push(std::thread::spawn(move || {
            if let Result::Ok(mut guard) = locker_clone.lock() {
                (*guard).push_str(&i.to_string());
                (*guard).push_str("  ");
                println!("guard: {:?}", guard);
            }
            println!("locker_clone = {:?}", locker_clone);
        }));
    }
    for handle in handles {
        handle.join();
    }

    println!("locker = {:?}", locker);
}
