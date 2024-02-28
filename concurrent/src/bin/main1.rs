fn main() {
    println!("*** *** Concurrent *** ***");

    let (sx, rx) = std::sync::mpsc::channel::<String>();
    let sx1 = sx.clone();
    let sx2 = sx.clone();

    let join_handle = std::thread::spawn(move || {
        for i in vec![10, 20, 30, 40] {
            let s = i.to_string();
            println!("===>>> s : {:?}", s);
            sx.send(s);
            std::thread::sleep(std::time::Duration::from_millis(100));
            // println!("===>>> s : {:?}", s); // borrow of moved value: `s`
        }
    });
    let join_handle1 = std::thread::spawn(move || {
        for i in vec![1010, 1020, 1030, 1040] {
            sx1.send(i.to_string());
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });
    let join_handle2 = std::thread::spawn(move || {
        for i in vec![10010, 10020, 10030, 10040] {
            sx2.send(i.to_string());
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    join_handle.join();
    join_handle1.join();
    join_handle2.join();

    while let Result::Ok(r) = rx.recv() {
        println!("RECV: r = {:?}", r);
    }
    println!("*** over ***");
}
