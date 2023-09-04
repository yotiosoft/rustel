use std::{net::{ToSocketAddrs, TcpStream}, io::BufRead, io::BufReader, io::Write, io::BufWriter};

fn read_something(reader: &mut BufReader<&TcpStream>) -> Result<String, std::io::Error> {
    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;
    Ok(buffer)
}

fn write_something(writer: &mut BufWriter<&TcpStream>, message: &str) -> Result<(), std::io::Error> {
    let mut msg = String::from(message);
    writer.write(message.as_bytes())?;
    writer.flush()?;
    Ok(())
}

fn main() {
    let host = "koukoku.shadan.open.ad.jp";
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
                println!("{}", str);
                //write_something(&mut writer, "Hello from the client!").unwrap();
            },
            Err(e) => println!("Failed to connect: {}", e),
        }
    } else {
        println!("No IPv4 address found");
    }
}
