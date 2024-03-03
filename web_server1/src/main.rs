use std::io::{Read, Write};

fn main() {
    println!("*** *** Web Server 1 *** ***");

    let addr = "0.0.0.0:8080";
    let listener = std::net::TcpListener::bind(addr).unwrap_or_else(|err| {
        println!("bind tcp addr failed: {err}, addr={addr}");
        std::process::exit(1);
    });

    for stream_result in listener.incoming() {
        if let Result::Err(err) = stream_result {
            println!("accept new connection failed: {err}");
            continue;
        }
        let stream = stream_result.unwrap();
        let addr_result = stream.peer_addr();
        if let Result::Err(err) = addr_result {
            println!("get remote addr failed: {err}");
            continue;
        }
        let addr = addr_result.unwrap();
        handle_stream(stream, addr);
    }
}

fn handle_stream(mut stream: std::net::TcpStream, addr: std::net::SocketAddr) {
    println!("accept new tcp connection: socket_addr={addr}");

    let mut buf = [0_u8; 1024];
    let req_data_result = stream.read(&mut buf[..]);
    if let Result::Err(err) = req_data_result {
        println!("read request data failed: {err}");
        return;
    }
    let req_data_len = req_data_result.unwrap();
    println!(
        "request data len = {req_data_len}, request data = \n{}",
        String::from_utf8_lossy(&buf[..req_data_len])
    );

    let (resp_status, resp_content) = if buf.starts_with(b"GET / HTTP/1.1\r\n") {
        ("HTTP/1.1 200 OK\r\n\r\n", "OK! Rust Web Server!")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "OOPS! Rust Web Server!")
    };
    let resp_data = format!("{}{}", resp_status, resp_content);
    let resp_write_result = stream.write(resp_data.as_bytes());
    if let Result::Err(err) = resp_write_result {
        println!("stream write response data failed: {err}, data={resp_data}");
        return;
    }
    let resp_write_len = resp_write_result.unwrap();
    println!("stream write response data: ok, len={resp_write_len}");

    let stream_flush_result = stream.flush();
    if let Result::Err(err) = stream_flush_result {
        println!("stream flush response data failed: {err}");
        return;
    }
}
