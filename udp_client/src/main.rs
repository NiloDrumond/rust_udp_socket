use std::{env, io, net::UdpSocket, process, time::Instant};

fn main() -> io::Result<()> {
    let mut args = env::args();
    args.next();

    let index: usize = match args.next() {
        Some(arg) => match arg.parse() {
            Ok(num) => num,
            Err(e) => {
                eprintln!("Index inválido: {}", e);
                process::exit(1);
            }
        },
        None => {
            eprintln!("Index não recebido");
            process::exit(1);
        }
    };

    let high_index = index / 10;
    let low_index = index % 10;
    let sock = UdpSocket::bind(format!("0.0.0.0:80{high_index}{low_index}"))?;

    let remote_addr = "127.0.0.1:8080";
    sock.connect(remote_addr)?;

    let num: u32 = 10_000_000;
    let durations = [0; 10_000];
    let durations = durations.map(|_| {
        sock.send_to(&num.to_be_bytes(), remote_addr).unwrap();

        let mut data = [0; 6];
        let now = Instant::now();
        let (size, _src) = sock.recv_from(&mut data).unwrap();
        let _response = &data[0..size];
        now.elapsed().as_millis()
    });
    let duration: u128 = durations.iter().sum::<u128>() / durations.len() as u128;
    println!("Average RTT: {}", duration);

    Ok(())
}
