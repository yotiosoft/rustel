use std::net::{ToSocketAddrs, TcpStream};
use std::io::{BufReader, Write, Read};
use encoding_rs;

enum Encode {
    UTF8,
    SHIFT_JIS,
}

enum IPv {
    IPv4,
    IPv6,
}

fn telnet_read_utf8(stream: &TcpStream) -> Result<String, std::io::Error> {
    let mut buf_reader = BufReader::new(stream);
    let mut buffer = String::new();
    buf_reader.read_to_string(&mut buffer)?;
    //println!("Received message: {:?} {}", buffer, text);
    Ok(buffer)
}

fn telnet_read_sjis(stream: &TcpStream) -> Result<String, std::io::Error> {
    let mut buf_reader = BufReader::new(stream);
    let mut buffer: [u8; 4] = [0; 4];
    buf_reader.read(&mut buffer)?;
    let (cow, _, _) = encoding_rs::SHIFT_JIS.decode(&buffer);
    let text = cow.into_owned();
    //println!("Received message: {:?} {}", buffer, text);
    Ok(text)
}

fn telnet_read(stream: &TcpStream, encode: &Encode) -> Result<String, std::io::Error> {
    match encode {
        Encode::UTF8 => telnet_read_utf8(stream),
        Encode::SHIFT_JIS => telnet_read_sjis(stream),
    }
}

fn main() {
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

        match TcpStream::connect(address) {
            Ok(stream) => {
                println!("Connected to the server!");
                loop {
                    let str = telnet_read(&stream, &encode).unwrap();
                    print!("{}", str);
                    std::io::stdout().flush().unwrap();
                    if str == "\0" {
                        break;
                    }
                }
            },
            Err(e) => println!("Failed to connect: {}", e),
        }
    } else {
        println!("No IPv4 address found");
    }
}
