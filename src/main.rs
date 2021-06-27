use std::env;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Read;
use std::io::Write;

mod udp;

fn handle_client(mut stream: TcpStream) {
    // read 128 bytes at a time from stream echoing back to stream
    loop {
        let mut read_buf = [0; 1024];
        match stream.read(&mut read_buf) {
            Ok(n) => {
                if n == 0 { 
                    // connection was closed
                    break;
                }
                stream.write(&read_buf[0..n]).unwrap();
            }
            Err(err) => {
                panic!(err);
            }
        }
    }
}

fn run_tcp() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // 中でacceptとかしてくれてるはず
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(_) => {
                println!("Error");
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 && args[1] == "udp" {
        println!("Run UDP echo server");
        udp::run();
    } else {
        println!("Run TCP echo server");
        run_tcp();
    }
}
