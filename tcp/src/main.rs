use std::net::{TcpListener};
use std::io::{Read};
use std::process;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut buff_size: u32 = 1024;
    if args.len() < 3 {
        eprintln!("Usage: {} <ip address x.x.x.x> <port> <buffsize (optional)>", args[0]);
        process::exit(1);
    }
    else if args.len() == 4{
        buff_size = args[3]
        .parse::<u32>()
        .unwrap_or_else(|_| {
            eprintln!("Error: Invalid buff size");
            1024
        });
    }
    
    let server_addr = format!("{}:{}", args[1], args[2]);
    let listener = TcpListener::bind(server_addr).unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

	let mut buffer = vec![0; buff_size as usize];
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
            	let client = stream.peer_addr().unwrap();
                println!("Received connection from {}", &client);
				loop {
	                match stream.read(&mut buffer) {
	                    Ok(size) => {
	                    	if size == 0{
	                    		println!("Conncetion closed by client: {}!", client);
	                    		//connection closed by client
	                    		break;
	                    	}
	                        let data = String::from_utf8_lossy(&buffer[..size]);
	                        println!("Received data: {}", data);
	                    },
	                    Err(e) => {
	                        eprintln!("Error receiving data: {}", e);
	                        break;
	                    }
                	}
                }
            },
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}
