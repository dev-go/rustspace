use std::io::{Read, Write};

fn main() {
    println!("*** *** Web Server 2 *** ***");

    let addr = "0.0.0.0:8080";
    let listener = std::net::TcpListener::bind(addr).unwrap_or_else(|err| {
        eprintln!("bind tcp listener addr failed: {err}, addr={addr}");
        std::process::exit(1);
    });

    for conn in listener.incoming() {
        match conn {
            Result::Err(err) => {
                eprintln!("accept new connection failed: {err}");
                continue;
            }
            Result::Ok(conn) => {
                std::thread::spawn(move || handle_stream(conn));
            }
        }
    }
}

fn handle_stream(mut conn: std::net::TcpStream) {
    let thread_id = std::thread::current().id();
    let remote_addr: std::net::SocketAddr;
    match conn.peer_addr() {
        Result::Err(err) => {
            eprintln!("get connection remote addr failed: {err}, thread_id={thread_id:?}");
            return;
        }
        Result::Ok(addr) => remote_addr = addr,
    }
    println!("handle stream: thread_id={thread_id:?}, remote_addr={remote_addr}");

    let mut buf = [0_u8; 1024];
    let req_data_len: usize;
    match conn.read(&mut buf[..]) {
        Result::Err(err) => {
            eprintln!("read request data failed: {err}, thread_id={thread_id:?}, remote_addr={remote_addr}");
            return;
        }
        Result::Ok(n) => req_data_len = n,
    }
    let req_data = &buf[..req_data_len];
    println!(
        "handle stream: thread_id={thread_id:?}, remote_addr={remote_addr}, request data:\n{}",
        String::from_utf8_lossy(req_data)
    );

    let resp_data = "HTTP/1.1 200 OK\r\n\r\n";
    if let Result::Err(err) = conn.write(resp_data.as_bytes()) {
        eprintln!(
            "write response data failed: {err}, thread_id={thread_id:?}, remote_addr={remote_addr}"
        );
        return;
    }
    if let Result::Err(err) = conn.flush() {
        eprintln!(
            "flush response data failed: {err}, thread_id={thread_id:?}, remote_addr={remote_addr}"
        );
        return;
    }
}
