fn main() {
    println!("*** *** Concurrent *** ***");
    let (sx, rx) = std::sync::mpsc::channel::<String>();
    let sx1 = sx.clone();
    let sx2 = sx.clone();

    let join1 = std::thread::spawn(move || {
        for i in vec![101, 102, 103, 104] {
            let send_result = sx1.send(i.to_string());
            println!("[THX1] i = {}, send_result = {:?}", i, send_result);
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });
    let join2 = std::thread::spawn(move || {
        for i in vec![201, 202, 203, 204] {
            let send_result = sx2.send(i.to_string());
            println!("[THX2] i = {}, send_result = {:?}", i, send_result);
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    std::thread::sleep(std::time::Duration::from_millis(200));
    drop(rx);
    println!("[MAIN] rx close");

    join1.join();
    join2.join();
}
