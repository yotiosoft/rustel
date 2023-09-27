use std::io::Write;
use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufReader, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use encoding_rs;
use super::args::Encode;

/// Recieve data from server and read it as UTF-8
async fn telnet_recv_utf8(stream: &mut ReadHalf<TcpStream>) -> Result<Option<String>, std::io::Error> {
    let mut buf_reader = BufReader::new(stream);
    let buffer = buf_reader.fill_buf().await?;
    if buffer.len() == 0 {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "No data reverived. Maybe connection closed."));
    }
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
    if buffer.len() == 0 {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "No data reverived. Maybe connection closed."));
    }
    if buffer[0] == 0 {
        return Ok(None);
    }
    let (cow, _, _) = encoding_rs::SHIFT_JIS.decode(&buffer);
    let text = cow.into_owned();
    //println!("Received message: {:?} {}", buffer, text);
    Ok(Some(text))
}

/// Recieve data from server and print it
pub async fn telnet_recv(mut stream: ReadHalf<TcpStream>, encode: Encode) -> Result<(), std::io::Error> {
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
    let buf_writer = stream;
    let (cow, _, _) = encoding_rs::SHIFT_JIS.encode(str);
    buf_writer.write(&cow).await?;
    buf_writer.flush().await?;
    Ok(())
}

/// Get input from stdin and send it to server
pub async fn telnet_send(mut stream: WriteHalf<TcpStream>, encode: Encode) -> Result<(), std::io::Error> {
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

/// Get input from message-string and send it to server
pub async fn telnet_send_message(mut stream: WriteHalf<TcpStream>, encode: Encode, message: String) -> Result<(), std::io::Error> {
    match encode {
        Encode::UTF8 => telnet_send_utf8(&mut stream, &message).await,
        Encode::SHIFTJIS => telnet_send_sjis(&mut stream, &message).await,
    }
}

/// Get input from message-string and send it to server (one character per one time)
pub async fn telnet_send_message_per_one_char(mut stream: WriteHalf<TcpStream>, encode: Encode, message: String, t: u64) -> Result<(), std::io::Error> {
    for c in message.chars() {
        match encode {
            Encode::UTF8 => telnet_send_utf8(&mut stream, &c.to_string()).await,
            Encode::SHIFTJIS => telnet_send_sjis(&mut stream, &c.to_string()).await,
        }?;
        // wait t millisecond
        tokio::time::sleep(std::time::Duration::from_millis(t)).await;
    }
    Ok(())
}
