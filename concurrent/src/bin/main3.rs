fn main() {
    println!("*** *** Concurrent *** ***");

    let (sx, rx) = std::sync::mpsc::channel::<String>();
    let sx1 = sx.clone();
    let sx2 = sx.clone();
    let join1 = std::thread::spawn(move || {
        for i in vec![101, 102, 103, 104] {
            let r = sx1.send(i.to_string());
            println!("[THREAD 1] i = {i}, r = {:?}", r);
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });
    let join2 = std::thread::spawn(move || {
        for i in vec![201, 202, 203, 204] {
            let r = sx2.send(i.to_string());
            println!("[THREAD 2] i = {i}, r = {:?}", r);
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    // // 一直等待, 不会退出
    // for i in rx {
    //     println!("[MAIN] receiv: i = {:?}", i);
    // }

    drop(sx);
    for i in rx {
        println!("[MAIN] receiv: i = {:?}", i);
    }
}
