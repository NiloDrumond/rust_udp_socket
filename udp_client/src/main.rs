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
        now.elapsed().as_millis() as i32
    });
    let mean_duration: i32 = durations.iter().sum::<i32>() / durations.len() as i32;
    let variance: f32 = durations
        .iter()
        .map(|value| {
            let diff: i32 = mean_duration - value;
            diff * diff
        })
        .sum::<i32>() as f32
        / durations.len() as f32;
    let std_deviation = variance.sqrt();
    println!("Average RTT: {}", mean_duration);
    println!("Standard Deviation: {}", std_deviation);

    Ok(())
}
