use std::io::{Read, Write};

fn main() {
    println!("*** *** Web Server 3 *** ***");

    let addr = "0.0.0.0:8080";
    let listener = std::net::TcpListener::bind(addr).unwrap_or_else(|err| {
        eprintln!("bind tcp listener addr failed: {err}");
        std::process::exit(1);
    });

    let cpu_cores = num_cpus::get();
    println!("cpu_cores = {cpu_cores}");
    let mut thread_pool = web_server3::ThreadPool::new(cpu_cores);

    for conn in listener.incoming() {
        match conn {
            Result::Err(err) => {
                eprintln!("accept new tcp connection failed: {err}");
            }
            Result::Ok(conn) => {
                thread_pool.run(|| handle_conn(conn));
            }
        }
    }
}

fn handle_conn(mut conn: std::net::TcpStream) {
    let thread_id = std::thread::current().id();
    println!("[handle conn] thread_id={thread_id:?}");
    let addr: std::net::SocketAddr;
    match conn.peer_addr() {
        Result::Err(err) => {
            eprintln!(
                "[handle conn] get connetion peer addr failed: {err}, thread_id={thread_id:?}"
            );
            return;
        }
        Result::Ok(v) => addr = v,
    };
    println!("[handle conn] thread_id={thread_id:?}, addr={addr}");

    let mut buf = [0_u8; 512];
    let req_data_len: usize;
    match conn.read(&mut buf[..]) {
        Result::Err(err) => {
            eprintln!("[handle conn] read request body failed: {err}, thread_id={thread_id:?}, addr={addr}");
            return;
        }
        Result::Ok(n) => req_data_len = n,
    }
    let req_data = &buf[..req_data_len];
    println!(
        "[handle conn] read request body: ok, thread_id={thread_id:?}, addr={addr}, req_data=\n{}",
        String::from_utf8_lossy(req_data)
    );

    if let Result::Err(err) = conn.write("HTTP/1.1 200 OK\r\n\r\nRust Web Server".as_bytes()) {
        eprintln!("[handle conn] write response data failed: {err}");
        return;
    }
    if let Result::Err(err) = conn.flush() {
        eprintln!("[handle conn] flush response data failed: {err}");
        return;
    }
}
