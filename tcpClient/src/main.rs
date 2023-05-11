use std::io::{self, Write};
use std::net::{TcpStream};
use std::env;
use std::process;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3{
        println!("Usage: {} <server IP> <server port>", args[0]);
        process::exit(1);
    }
    let server = format!("{}:{}", args[1], args[2]);
    // Connect to the host
    let mut stream = TcpStream::connect(server)?;

    println!("Enter the message (type 'quit' to finish): ");
    let mut message = String::new();
    loop {
        io::stdin()
            .read_line(&mut message)
            .expect("Couldn't read from stdio");
        // Send packets to the host
        
        if message == "quit\n" {
            break;
        }
        stream.write(message.as_bytes())?;
        message.clear();
    }


    drop(stream);
    Ok(())
}
