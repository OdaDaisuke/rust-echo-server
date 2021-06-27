use std::net::UdpSocket;
pub fn run() -> std::io::Result<()> {
    unsafe {
        let addr = "127.0.0.1:34254";
        let mut socket = UdpSocket::bind(addr).expect("couldn't bind to address");
        println!("UDP lisning: {}", addr);

        // Receives a single datagram message on the socket.
        let mut buf = [0; 10];
        let (amt, src) = socket.recv_from(&mut buf)?;
        
        socket.send_to(&buf, &src)?;
    }
    Ok(())
}
