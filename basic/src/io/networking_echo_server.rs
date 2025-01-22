use std::net::TcpListener;
use std::io;
use std::thread::spawn;

/// Accept connections forever, spawning a thread for each one
fn echo_main(addr: &str) -> io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("Echo server listening on {}", addr);
    loop {
        // block the calling thread until a new client TCP connection is established
        let (mut stream, addr) = listener.accept()?;
        println!("Got connection from {}", addr);

        // spawn thread to handle this client
        let mut write_stream = stream.try_clone()?;
        spawn(move || {
            // echo received thing back to the connection
            io::copy(&mut stream, &mut write_stream)
                .expect("Failed to write to client stream!");
            println!("Connection terminated");
        });
    }
}

fn main() {
    echo_main("127.0.0.1:1080").unwrap();
}

// Note: for high-perf server use async input and output
