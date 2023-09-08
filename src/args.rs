use clap::Parser;

#[derive(Clone)]
pub enum Encode {
    UTF8,
    SHIFTJIS,
}

pub enum IPv {
    IPv4,
    IPv6,
}

pub struct ArgStruct {
    pub url: String,
    pub port: u16,
    pub encode: Encode,
    pub ipv: IPv,
}

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // destination URL (required)
    #[arg(short, long)]
    url: String,

    // destination port number (default: 23)
    #[arg(short, long, default_value = "23")]
    port: u16,

    // encode (utf8 or shift_jis; default: utf8)
    #[arg(short, long, default_value = "utf8")]
    encode: String,

    // IP version (4 or 6; default: 4)
    #[arg(short, long, default_value = "4")]
    ipv: u8,
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

    ArgStruct {
        url: args.url,
        port: args.port,
        encode: encode,
        ipv: ipv,
    }
}
