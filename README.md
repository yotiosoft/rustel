# rustel

# 概要

Rust で書いた TELNET 用クライアントです。  
IPv4 および IPv6 に、文字コードは UTF-8 および Shift_JIS に対応しています。

# インストール

```bash
$ git clone https://github.com/yotiosoft/rustel.git
$ cargo install --path .
```

# 使用方法

```bash
$ rustel -u [URL] -p [Port Number] -e [Encode (utf8 or sjis)] -i [IP version (4 or 6)]
```

既定値：

- Port Number : 23
- Encode : utf8
- IP version : 4

例：

```bash
$ rustel -u koukoku.shadan.open.ad.jp -p 23 -e sjis
```
