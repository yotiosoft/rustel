use std::{net::{ToSocketAddrs, TcpStream}, io::BufRead, io::BufReader, io::Write, io::BufWriter};

fn read_something(reader: &mut BufReader<&TcpStream>) -> Result<Vec<u8>, std::io::Error> {
    let mut buffer = Vec::new();
    reader.read_until(b'\n', &mut buffer)?;
    for line in reader.lines() {
        println!("RECV: {:?}", line?);
    }
    Ok(buffer)
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
                let mut reader = BufReader::new(&stream);
                let mut writer = BufWriter::new(&stream);
                
                let str = read_something(&mut reader).unwrap();
                write_u64(&mut writer, 0xFFFC24).unwrap();
            },
            Err(e) => println!("Failed to connect: {}", e),
        }
    } else {
        println!("No IPv4 address found");
    }
}
