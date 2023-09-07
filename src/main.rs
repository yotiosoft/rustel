use std::io::Write;
use std::net::ToSocketAddrs;

use tokio::io::{AsyncReadExt, AsyncWriteExt, AsyncBufReadExt, BufReader, ReadHalf, WriteHalf};
use tokio::net::TcpStream;

use encoding_rs;

const TCP_LEN_MAX: usize = 65536;

#[derive(Clone)]
enum Encode {
    UTF8,
    SHIFT_JIS,
}

enum IPv {
    IPv4,
    IPv6,
}

async fn telnet_read_utf8(stream: &mut ReadHalf<TcpStream>) -> Result<Option<String>, std::io::Error> {
    let mut buf_reader = BufReader::new(stream);
    let mut buffer = String::new();
    buf_reader.read_to_string(&mut buffer).await?;
    //println!("Received message: {}", buffer);
    if buffer.len() == 0 {
        return Ok(None);
    }
    Ok(Some(buffer))
}

async fn telnet_read_sjis(stream: &mut ReadHalf<TcpStream>) -> Result<Option<String>, std::io::Error> {
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

async fn telnet_read(mut stream: ReadHalf<TcpStream>, encode: Encode) -> Result<(), std::io::Error> {
    loop {
        let str = match encode {
            Encode::UTF8 => telnet_read_utf8(&mut stream).await?,
            Encode::SHIFT_JIS => telnet_read_sjis(&mut stream).await?,
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

async fn telnet_write_utf8(stream: &mut WriteHalf<TcpStream>, str: &str) -> Result<(), std::io::Error> {
    let buf_writer = stream;
    buf_writer.write(str.as_bytes());
    buf_writer.flush();
    Ok(())
}

async fn telnet_write_sjis(stream: &mut WriteHalf<TcpStream>, str: &str) -> Result<(), std::io::Error> {
    let buf_writer = stream;
    let (cow, _, _) = encoding_rs::SHIFT_JIS.encode(str);
    buf_writer.write(&cow);
    buf_writer.flush();
    Ok(())
}

async fn telnet_write(stream: &mut WriteHalf<TcpStream>, encode: &Encode, str: &str) -> Result<(), std::io::Error> {
    match encode {
        Encode::UTF8 => telnet_write_utf8(stream, str).await,
        Encode::SHIFT_JIS => telnet_write_sjis(stream, str).await,
    }
}

async fn telnet_input(mut stream: WriteHalf<TcpStream>, encode: Encode) -> Result<(), std::io::Error> {
    loop {
        let mut input = String::new();
        if let Ok(_) = std::io::stdin().read_line(&mut input) {
            if input.len() == 0 {
                break;
            }
            telnet_write(&mut stream, &encode, &input).await?;
        }
        else {
            break;
        }
    };
    Ok(())
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
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
            Ok(stream) => {
                println!("Connected to the server!");
                let (reader, writer) = tokio::io::split(stream);

                // read
                let reader = tokio::spawn(telnet_read(reader, encode.clone()));

                // write
                let writer = tokio::spawn(telnet_input(writer, encode.clone()));

                reader.await?;
                writer.abort();
            },
            Err(e) => println!("Failed to connect: {}", e),
        }
    } else {
        println!("No IPv4 address found");
    }

    Ok(())
}
