use std::io::Result;
use std::env;
use std::net::ToSocketAddrs;



#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("Usage: {} <IP to listen> <port to listen> 
                <server mode (onetoone or onetomany)>", args[0]);
        return Ok(());
    }
    let server_addr = format!("{}:{}", args[1], args[2]);

    let server_mode: &String = &args[3];

    match server_addr.to_socket_addrs() {
        Ok(mut addr) => {
            if let Some(address) = addr.next() {

                let server_socket;
                if server_mode == "onetomany"{
                    server_socket = sctp_rs::Socket::new_v4(sctp_rs::SocketToAssociation::OneToMany)?;
                } else{
                    server_socket = sctp_rs::Socket::new_v4(sctp_rs::SocketToAssociation::OneToOne)?;
                }
                server_socket.sctp_bindx(&[address], sctp_rs::BindxFlags::Add)?;
                let server_socket = server_socket.listen(100)?;
                
                let mut accepted: Option<sctp_rs::ConnectedSocket> = None;
                loop {
                    if server_mode != "onetomany"{
                        let (_accepted, client_address) = server_socket.accept().await?;
                        accepted = Some(_accepted);
                        println!("Client connected: {}", client_address);
                    }
                    loop {
                        let recieved: sctp_rs::NotificationOrData;
                        if server_mode == "onetomany"{
                            recieved = server_socket.sctp_recv().await?;
                        } else{
                            recieved = accepted.as_ref().unwrap().sctp_recv().await?;
                        }

                        if let sctp_rs::NotificationOrData::Data(mut data) = recieved{
                            if data.payload.is_empty() {
                                break;
                            }
                            let msg = String::from_utf8_lossy(&data.payload);                       
                            println!("Recieved: {:?}", msg);
                            data.payload = Vec::new();
                        }
                        
                    }
                }

            } else{
                panic!("Failed to parse address!");
            }
        },
        Err(err) => {
            panic!("Failed to parse address: {}", err);
        }
    };

}
