//! Handle UDP packets

use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};
use std::net::{IpAddr, UdpSocket};

/// Create an UDP socket and handle messages
pub fn listen(address: IpAddr, port: u16, output_file: &str) -> io::Result<()> {
    let socket = UdpSocket::bind((address, port))?;
    println!("Listening on UDP, address {}, port {}", address, port);

    let mut buf = [0; 2048];
    let mut writer = BufWriter::new(
        File::options()
            .append(true)
            .create(true)
            .open(output_file)?,
    );
    writeln!(writer, "=============================")?;

    while let Ok((n, src)) = socket.recv_from(&mut buf) {
        if &buf[..3] == b"PFT" {
            // PLEASE FLUSH, THANKS
            writer.flush()?;
            continue;
        }
        println!("Received {} bytes from {}", n, src);

        write!(writer, "[{}] ", src)?;
        writer.write(&buf[..n])?;
    }

    Ok(())
}
