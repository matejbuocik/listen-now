//! Handle TCP connections

use std::io;
use std::io::{Read, Write};
use std::net::{IpAddr, TcpListener, TcpStream};
use threadpool::ThreadPool;

/// Create a TCP socket and handle incoming connections
pub fn listen(address: IpAddr, port: u16, num_threads: usize) -> io::Result<()> {
    let listener = TcpListener::bind((address, port))?;
    println!("Listening on TCP, address {}, port {}", address, port);

    let pool = ThreadPool::new(num_threads);

    for stream in listener.incoming() {
        pool.execute(move || {
            handle_connection(stream.unwrap()).unwrap();
        });
    }

    Ok(())
}

/// Handle incoming connection
fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0; 1024];

    while let Ok(n) = stream.read(&mut buf) {
        if n == 0 {
            println!("EOF");
            break;
        }

        println!("{} bytes read from {}", n, stream.peer_addr()?);
        io::stdout().write(&buf[..n])?;
        println!("-------------------------");
        stream.write(b"hello")?;
    }

    println!("=============================================");
    Ok(())
}
