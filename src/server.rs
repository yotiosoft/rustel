use std::io::Write;
use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufReader, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use encoding_rs;
use super::args::Encode;

/// Get input from stdin and send it to server
pub async fn telnet_send(mut str_buf: &String, encode: Encode) -> Result<(), std::io::Error> {
    loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                /*
                let str = match encode {
                    Encode::UTF8 => input.as_mut_vec().into_iter().map(|&x| x as char).collect::<String>(),
                    Encode::SHIFTJIS => {
                        let (cow, _, _) = encoding_rs::SHIFT_JIS.encode(&input);
                        cow.into_owned()
                    },
                };
                */
                let buf_writer = &mut std::io::stdout();
                buf_writer.write(str_buf.as_bytes())?;
                buf_writer.flush()?;
            },
            Err(e) => {
                return Err(e);
            }
        }
    };
}
