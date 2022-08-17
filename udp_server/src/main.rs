use std::{
    io,
    net::{SocketAddr, UdpSocket},
    thread,
};

fn find_nth_odd(n: u32) -> u32 {
    let mut i = 0;
    let mut odd_count = 0;

    while odd_count != n {
        i += 1;
        if i % 2 == 1 {
            odd_count += 1;
        }
    }

    i
}

fn handle_client(n: u32, sock: UdpSocket, addr: SocketAddr) {
    let nth_odd = find_nth_odd(n);
    sock.send_to(&nth_odd.to_be_bytes(), addr).unwrap();
}

fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:8080")?;
    let mut buf = [0; 1024];
    loop {
        let (size, addr) = sock.recv_from(&mut buf)?;

        let mut num = [0u8; 4];
        num.clone_from_slice(&buf[0..size]);
        let num = u32::from_be_bytes(num);
        let sock_clone = sock.try_clone()?;
        thread::spawn(move || {
            handle_client(num, sock_clone, addr);
        });
    }
}
