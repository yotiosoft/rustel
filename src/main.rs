use std::net::ToSocketAddrs;
use tokio::net::{TcpStream, TcpListener};
use args::{IPv, Encode};
use std::io::Read;
use std::fs::File;

mod args;
mod client;

async fn client(host: String, port: u16, encode: args::Encode, ipv: IPv) -> Result<(), std::io::Error> {
    let host_and_port = format!("{}:{}", host, port);
    let mut addresses = host_and_port.to_socket_addrs()?;

    let address = match ipv {
        IPv::IPv4 => addresses.find(|x| x.is_ipv4()),
        IPv::IPv6 => addresses.find(|x| x.is_ipv6()),
    };
    if let Some(address) = address {
        println!("Found an IPv4 address: {}", address);

        match TcpStream::connect(address).await {
            Ok(stream) => {
                println!("Connected to the server!");
                let address = stream.peer_addr()?;
                let (reader, writer) = tokio::io::split(stream);

                // read
                let reader = tokio::spawn(client::telnet_recv(reader, encode.clone()));

                // write
                let writer = tokio::spawn(client::telnet_send(writer, encode.clone()));

                let _ = reader.await?;
                writer.abort();
                println!("\nConnection with {address} closed.");
                Ok(())
            },
            Err(e) => {
                println!("Failed to connect: {}", e);
                Err(e)
            }
        }
    } else {
        println!("No address found");
        Err(std::io::Error::new(std::io::ErrorKind::Other, "No address found"))
    }
}

fn read_file(path: &str) -> Result<String, std::io::Error> {
    if !std::path::Path::new(path).exists() {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "File not found"));
    }

    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

async fn server(host: String, port: u16, encode: args::Encode, ipv: IPv, server_one_char: bool, server_message: Option<String>, wait_ms: u64) -> Result<(), std::io::Error> {
    let host_and_port = format!("{}:{}", host, port);
    let mut addresses = host_and_port.to_socket_addrs()?;

    let address = match ipv {
        IPv::IPv4 => addresses.find(|x| x.is_ipv4()),
        IPv::IPv6 => addresses.find(|x| x.is_ipv6()),
    };

    let listener = TcpListener::bind(address.unwrap()).await?;
    loop {
        let (mut stream, _) = listener.accept().await?;
        let encode_clone = encode.clone();
        let server_message = server_message.clone();
        tokio::spawn(async move {
            let addr = stream.peer_addr().unwrap();
            println!("Accepted connection from: {}", addr);
            let (reader, writer) = tokio::io::split(stream);

            // read
            let reader = tokio::spawn(client::telnet_recv(reader, encode_clone.clone()));

            // write
            let writer = if let Some(server_message) = server_message {
                if server_one_char {
                    tokio::spawn(client::telnet_send_message_per_one_char(writer, encode_clone.clone(), server_message, wait_ms))
                }
                else {
                    tokio::spawn(client::telnet_send_message(writer, encode_clone.clone(), server_message))
                }
            }
            else {
                tokio::spawn(client::telnet_send(writer, encode_clone.clone()))
            };

            let _ = reader.await;
            writer.abort();
            println!("Connection with {} closed.", addr);
        });
    }
    
    Ok(())
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let args = args::parser();
    let mode = args.mode;
    let host = args.url;
    let port = args.port;
    let encode = args.encode;
    let ipv = args.ipv;
    let server_one_char = args.server_one_char;
    let server_wait_ms = args.server_wait_ms;
    let server_message = args.server_message;
    let server_message_file = args.server_message_file;
    //let host = "koukoku.shadan.open.ad.jp";
    //let host = "india.colorado.edu";
    //let port = 23;
    //let encode = Encode::SHIFTJIS;
    //let ipv = IPv::IPv4;

    match mode {
        args::Mode::Client => {
            client(host, port, encode, ipv).await?;
        },
        args::Mode::Server => {
            if let Some(server_message_file) = server_message_file {
                let server_message = read_file(server_message_file.to_str().unwrap())?;
                server(host, port, encode, ipv, server_one_char, Some(server_message), server_wait_ms).await?;
            }
            else {
                server(host, port, encode, ipv, server_one_char, server_message, server_wait_ms).await?;
            }
        },
    }

    Ok(())
}
