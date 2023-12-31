use clap::Parser;
use std::path::PathBuf;

#[derive(Clone)]
pub enum Encode {
    UTF8,
    SHIFTJIS,
}

pub enum IPv {
    IPv4,
    IPv6,
}

pub enum Mode {
    Client,
    Server,
}

pub struct ArgStruct {
    pub mode: Mode,
    pub url: String,
    pub port: u16,
    pub encode: Encode,
    pub ipv: IPv,
    pub server_one_char: bool,
    pub server_wait_ms: u64,
    pub server_message: Option<String>,
    pub server_message_file: Option<PathBuf>,
}

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// set as server mode
    #[arg(short, long)]
    server: bool,

    /// set as client mode (default)
    #[arg(short, long)]
    client: bool,

    /// destination URL (required)
    #[arg(short, long)]
    url: String,

    /// destination port number (default: 23)
    #[arg(short, long, default_value = "23")]
    port: u16,

    /// encode (utf8 or shift_jis; default: utf8)
    #[arg(short, long, default_value = "utf8")]
    encode: String,

    /// IP version (4 or 6; default: 4)
    #[arg(short, long, default_value = "4")]
    ipv: u8,

    /// send one character to client (server mode only)
    #[arg(short, long)]
    one_char: bool,

    /// wait time for sending the message (millisecond) (server mode only; default: 100)
    #[arg(short, long, default_value = "100")]
    wait_ms: u64,

    /// message to send to client (server mode only)
    #[arg(short, long)]
    message: Option<String>,

    /// message file to send to client (server mode only)
    #[arg(short, long)]
    file: Option<PathBuf>,
}

pub fn parser() -> ArgStruct {
    let args = Args::parse();

    let encode = match args.encode.to_lowercase().as_str() {
        "utf8" => Encode::UTF8,
        "shift_jis" => Encode::SHIFTJIS,
        "sjis" => Encode::SHIFTJIS,
        _ => Encode::UTF8,
    };

    let ipv = match args.ipv {
        4 => IPv::IPv4,
        6 => IPv::IPv6,
        _ => IPv::IPv4,
    };

    let mode = match (args.server, args.client) {
        (true, false) => Mode::Server,
        (false, true) => Mode::Client,
        _ => Mode::Client,      // default
    };

    ArgStruct {
        mode: mode,
        url: args.url,
        port: args.port,
        encode: encode,
        ipv: ipv,
        server_one_char: args.one_char,
        server_wait_ms: args.wait_ms,
        server_message: args.message,
        server_message_file: args.file,
    }
}
