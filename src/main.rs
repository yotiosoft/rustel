use std::{net::{ToSocketAddrs, TcpStream}, io::BufReader, io::Write, io::{BufWriter, Read}};
use encoding_rs;

fn telnet_read(stream: &TcpStream) -> Result<String, std::io::Error> {
    let mut buf_reader = BufReader::new(stream);
    let mut buffer: [u8; 2] = [0; 2];
    buf_reader.read(&mut buffer)?;
    //println!("Received message: {:?}", buffer);
    let (cow, _, _) = encoding_rs::SHIFT_JIS.decode(&buffer);
    let text = cow.into_owned();
    std::io::stdout().flush()?;
    Ok(text)
}

fn main() {
    let host = "koukoku.shadan.open.ad.jp";
    //let host = "india.colorado.edu";
    let port = 23;

    let host_and_port = format!("{}:{}", host, port);
    let mut addresses = host_and_port.to_socket_addrs().unwrap();

    if let Some(address) = addresses.find(|x| x.is_ipv4()) {
        println!("Found an IPv4 address: {}", address);

        match TcpStream::connect(address) {
            Ok(stream) => {
                println!("Connected to the server!");
                loop {
                    let str = telnet_read(&stream).unwrap();
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



fn write_something(writer: &mut BufWriter<&TcpStream>, message: &str) -> Result<(), std::io::Error> {
    let msg = String::from(message);
    println!("Sending message: {}", msg);
    writer.write(msg.as_bytes())?;
    writer.flush()?;
    Ok(())
}

fn write_u64(writer: &mut BufWriter<&TcpStream>, message: u64) -> Result<(), std::io::Error> {
    println!("Sending message: {}", message);
    writer.write(message.to_string().as_bytes())?;
    writer.flush()?;
    Ok(())
}
