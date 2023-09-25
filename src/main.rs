use std::net::ToSocketAddrs;
use tokio::net::TcpStream;
use args::IPv;

mod args;
mod client;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let args = args::parser();
    let host = args.url;
    let port = args.port;
    let encode = args.encode;
    let ipv = args.ipv;
    //let host = "koukoku.shadan.open.ad.jp";
    //let host = "india.colorado.edu";
    //let port = 23;
    //let encode = Encode::SHIFTJIS;
    //let ipv = IPv::IPv4;

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
                let (reader, writer) = tokio::io::split(stream);

                // read
                let reader = tokio::spawn(client::telnet_recv(reader, encode.clone()));

                // write
                let writer = tokio::spawn(client::telnet_send(writer, encode.clone()));

                let _ = reader.await?;
                writer.abort();
            },
            Err(e) => println!("Failed to connect: {}", e),
        }
    } else {
        println!("No IPv4 address found");
    }

    Ok(())
}
