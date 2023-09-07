use std::io::Write;
use std::net::ToSocketAddrs;

use tokio::net::TcpStream;
use tokio::io::{BufReader, AsyncWrite, AsyncRead, AsyncReadExt, AsyncWriteExt};
use encoding_rs;

const TCP_LEN_MAX: usize = 65536;

enum Encode {
    UTF8,
    SHIFT_JIS,
}

enum IPv {
    IPv4,
    IPv6,
}

async fn telnet_read_utf8(stream: &mut TcpStream) -> Result<Option<String>, std::io::Error> {
    let mut buf_reader = BufReader::new(stream);
    let mut buffer = String::new();
    buf_reader.read_to_string(&mut buffer).await;
    //println!("Received message: {:?} {}", buffer, text);
    if buffer.len() == 0 {
        return Ok(None);
    }
    Ok(Some(buffer))
}

async fn telnet_read_sjis(stream: &mut TcpStream) -> Result<Option<String>, std::io::Error> {
    println!("telnet_read_sjis");
    let mut buf_reader = BufReader::new(stream);
    let mut buffer: [u8; TCP_LEN_MAX] = [0; TCP_LEN_MAX];
    buf_reader.read(&mut buffer).await?;
    if buffer[0] == 0 {
        return Ok(None);
    }
    let (cow, _, _) = encoding_rs::SHIFT_JIS.decode(&buffer);
    let text = cow.into_owned();
    //println!("Received message: {:?} {}", buffer, text);
    Ok(Some(text))
}

async fn telnet_read(stream: &mut TcpStream, encode: &Encode) -> Result<Option<String>, std::io::Error> {
    match encode {
        Encode::UTF8 => telnet_read_utf8(stream).await,
        Encode::SHIFT_JIS => telnet_read_sjis(stream).await,
    }
}

async fn telnet_write_utf8(stream: &mut TcpStream, str: &str) -> Result<(), std::io::Error> {
    let mut buf_writer = stream;
    buf_writer.try_write(str.as_bytes())?;
    buf_writer.flush();
    Ok(())
}

async fn telnet_write_sjis(stream: &mut TcpStream, str: &str) -> Result<(), std::io::Error> {
    let mut buf_writer = stream;
    let (cow, _, _) = encoding_rs::SHIFT_JIS.encode(str);
    buf_writer.try_write(&cow)?;
    buf_writer.flush();
    Ok(())
}

async fn telnet_write(stream: &mut TcpStream, encode: &Encode, str: &str) -> Result<(), std::io::Error> {
    match encode {
        Encode::UTF8 => telnet_write_utf8(stream, str).await,
        Encode::SHIFT_JIS => telnet_write_sjis(stream, str).await,
    }
}

async fn telnet_input(stream: &mut TcpStream, encode: &Encode) -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    if let Ok(_) = std::io::stdin().read_line(&mut input) {
        if input.len() == 0 {
            return Ok(());
        }
        telnet_write(stream, encode, &input).await?;
    }
    else {
        return Ok(());
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let host = "koukoku.shadan.open.ad.jp";
    //let host = "india.colorado.edu";
    let port = 23;
    let encode = Encode::SHIFT_JIS;
    let ipv = IPv::IPv4;

    let host_and_port = format!("{}:{}", host, port);
    let mut addresses = host_and_port.to_socket_addrs().unwrap();

    let address = match ipv {
        IPv::IPv4 => addresses.find(|x| x.is_ipv4()),
        IPv::IPv6 => addresses.find(|x| x.is_ipv6()),
    };
    if let Some(address) = address {
        println!("Found an IPv4 address: {}", address);

        match TcpStream::connect(address).await {
            Ok(mut stream) => {
                println!("Connected to the server!");
                loop {
                    // read
                    let str = telnet_read(&mut stream, &encode).await.unwrap();
                    if let Some(str) = str {
                        print!("{}", str);
                        std::io::stdout().flush().unwrap();
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

                    // write
                    telnet_input(&mut stream, &encode).await.unwrap()
                }
            },
            Err(e) => println!("Failed to connect: {}", e),
        }
    } else {
        println!("No IPv4 address found");
    }
}
