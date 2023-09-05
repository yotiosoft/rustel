use std::{net::{ToSocketAddrs, TcpStream}, io::BufRead, io::BufReader, io::Write, io::{BufWriter, Read}};

fn read_something(mut stream: &TcpStream) -> Result<(), std::io::Error> {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut buffer: [u8; 1024] = [0; 1024];
    buf_reader.read(&mut buffer)?;
    println!("Received message: {:?}", buffer);
    Ok(())
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
                let str = read_something(&stream).unwrap();
                //write_u64(&mut writer, 0xFFFC24).unwrap();
            },
            Err(e) => println!("Failed to connect: {}", e),
        }
    } else {
        println!("No IPv4 address found");
    }
}
