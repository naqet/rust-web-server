use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use rust_web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("localhost:3000").expect("Listener failed");
    let pool = ThreadPool::build(5).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        })
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let req_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, content_file) = match &req_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let content = fs::read_to_string(content_file).expect("Couldn't read file");
    let length = content.len();
    let res = format!(
        "{status_line}\r\nContent-Length: {length}\rContent-Type: text/html\r\n\r\n{content}"
    );
    stream.write_all(res.as_bytes()).unwrap();
}
