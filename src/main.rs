use std::env;

mod tcp;
mod udp;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 && args[1] == "udp" {
        println!("Run UDP echo server");
        udp::run();
    } else {
        println!("Run TCP echo server");
        tcp::run();
    }
}
