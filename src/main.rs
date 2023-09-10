use std::io::Write;
use std::net::ToSocketAddrs;

use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufReader, ReadHalf, WriteHalf};
use tokio::net::TcpStream;

use encoding_rs;

mod args;
use args::{Encode, IPv};

/// Recieve data from server and read it as UTF-8
async fn telnet_recv_utf8(stream: &mut ReadHalf<TcpStream>) -> Result<Option<String>, std::io::Error> {
    let mut buf_reader = BufReader::new(stream);
    let buffer = buf_reader.fill_buf().await?;
    //println!("Received message: {}", buffer);
    if buffer.len() == 0 {
        return Ok(None);
    }
    Ok(Some(buffer.iter().map(|&x| x as char).collect::<String>()))
}

/// Recieve data from server and read it as Shift-JIS
async fn telnet_recv_sjis(stream: &mut ReadHalf<TcpStream>) -> Result<Option<String>, std::io::Error> {
    let mut buf_reader = BufReader::new(stream);
    let buffer = buf_reader.fill_buf().await?;
    if buffer[0] == 0 {
        return Ok(None);
    }
    let (cow, _, _) = encoding_rs::SHIFT_JIS.decode(&buffer);
    let text = cow.into_owned();
    //println!("Received message: {:?} {}", buffer, text);
    Ok(Some(text))
}

/// Recieve data from server and print it
async fn telnet_recv(mut stream: ReadHalf<TcpStream>, encode: Encode) -> Result<(), std::io::Error> {
    loop {
        let str = match encode {
            Encode::UTF8 => telnet_recv_utf8(&mut stream).await?,
            Encode::SHIFTJIS => telnet_recv_sjis(&mut stream).await?,
        };

        if let Some(str) = str {
            print!("{}", str);
            std::io::stdout().flush()?;
            if str == "\0" {
                break;
            }
            // is buffer contains EOF?
            if str.contains("\u{1a}") {
                break;
            }
        }
        else {
            break;
        }
    };
    Ok(())
}

/// Get input from stdin and send it to server as UTF-8
async fn telnet_send_utf8(stream: &mut WriteHalf<TcpStream>, str: &str) -> Result<(), std::io::Error> {
    let buf_writer = stream;
    buf_writer.write(str.as_bytes()).await?;
    buf_writer.flush().await?;
    Ok(())
}

/// Get input from stdin and send it to server as Shift-JIS
async fn telnet_send_sjis(stream: &mut WriteHalf<TcpStream>, str: &str) -> Result<(), std::io::Error> {
    println!("send: {}", str);
    let buf_writer = stream;
    let (cow, _, _) = encoding_rs::SHIFT_JIS.encode(str);
    buf_writer.write(&cow).await?;
    buf_writer.flush().await?;
    Ok(())
}

/// Get input from stdin and send it to server
async fn telnet_send(mut stream: WriteHalf<TcpStream>, encode: Encode) -> Result<(), std::io::Error> {
    loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                match encode {
                    Encode::UTF8 => telnet_send_utf8(&mut stream, &input).await,
                    Encode::SHIFTJIS => telnet_send_sjis(&mut stream, &input).await,
                }?;
            },
            Err(e) => {
                return Err(e);
            }
        }
    };
}

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
                let reader = tokio::spawn(telnet_recv(reader, encode.clone()));

                // write
                let writer = tokio::spawn(telnet_send(writer, encode.clone()));

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
