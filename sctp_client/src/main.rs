use std::env;
use std::net::ToSocketAddrs;
use std::io::{Result,  self};


#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: {} <server IP> <server port>", args[0]);
        return Ok(());
    }
    let server_addr = format!("{}:{}", args[1], args[2]);

    match server_addr.to_socket_addrs(){
        Ok(mut addr) => {
            if let Some(address) = addr.next(){

                let client_socket = sctp_rs::Socket::new_v4(sctp_rs::SocketToAssociation::OneToOne)?;
                let (connected, assoc_id) = client_socket.sctp_connectx(&[address]).await?;
                //println!("Connected: {:#?}, assoc_id: {}", connected, assoc_id);
                println!("Connected to server: {}", address);
                println!("Enter the message: (type 'quit' to finish)");
                let mut message = String::new();
                loop {
                    io::stdin()
                        .read_line(&mut message)
                        .expect("Couldn't read from stdio");
                    if message == "quit\n"{
                        break;
                    }
                    let send_data: sctp_rs::SendData = sctp_rs::SendData {
                        payload: message.as_bytes().to_vec(),
                        snd_info: None,
                    };
                    connected.sctp_send(send_data).await?;
                }

            } else{
                panic!("Failed to parse address!");
            }
        },
        Err(err) => {
            panic!("Failed to parse address: {}", err);
        }
    };
    Ok(())
    
}
