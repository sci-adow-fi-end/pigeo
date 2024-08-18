use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::mpsc;
use std::thread;
//use std::time::Duration;

fn main() -> std::io::Result<()> {
    let (tx, rx) = mpsc::channel();

    let client = thread::spawn(move || {
        println!("il client è nato");
        let _response: bool = rx.recv().unwrap();
        let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();

        let message = b"yabadabadooo!";
        stream.write_all(message).unwrap();

        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).unwrap();

        println!(
            "Risposta dal server: {}",
            buffer
                .iter()
                .take(bytes_read)
                .map(|&n| n as char)
                .collect::<String>()
        );

        println!("ho finitoo");
    });

    let server = thread::spawn(move || {
        println!("il server è nato");
        let ready: bool = true;
        tx.send(ready).unwrap();

        let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();

            let mut buffer = [0; 1024]; // buffer di 1024 byte
            let bytes_read = stream.read(&mut buffer).unwrap();

            println!(
                "Ricevuti {} byte: {}",
                bytes_read,
                buffer
                    .iter()
                    .take(bytes_read)
                    .map(|&n| n as char)
                    .collect::<String>()
            );

            let response = b"message saved";
            stream.write_all(response).unwrap();

            println!(
                "Risposta inviata: {}",
                buffer
                    .iter()
                    .take(bytes_read)
                    .map(|&n| n as char)
                    .collect::<String>()
            );
        }
    });
    server.join().unwrap();
    client.join().unwrap();
    //stream.write(&[1])?;
    //stream.read(&mut [0; 128])?;
    Ok(())
}
