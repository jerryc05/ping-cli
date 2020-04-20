# ping-cli

A PING emulator (for `Linux`-like OS) for Cloudflare 2020 Internship Coding Challenge.

Please refer to https://github.com/jerryc05/ping-cli/tree/dev-rust for latest version.

[toc]

## Usage

```text
SYNOPSIS
	ping-cli ADDR_OR_HOST [OPTION_1 [OPTION_2 [...]]]

OPTIONS
  -c count
		Number of PINGs to send. 
		Type: usize
  -i interval
		Number of PINGs to send.
		Type: f32, [>= 0]
  -s packetsize
		Size of packet (payload) to send.
		Type: u16
  -t ttl
		Time to live.
		Type: u32
  -W timeout
		Timeout for each PING to end (as Request timed out).
		Type: f32, [>= 0]
```



## Implementation Detail

- Language: [Rust](https://www.rust-lang.org/)
- Dependencies:
  - [socket2](https://docs.rs/socket2)
- Supported Specifications:
  - ICMPv4
  - ICMPv6
  - DNS Resolve using `host` utility from `Linux` systems.
  - `-c count` flag from `man ping` page.
  - `-i interval` flag from `man ping` page.
  - `-s packetsize` flag from `man ping` page.
  - `-t ttl` flag from `man ping` page.
  - `-W timeout` flag from `man ping` page, except that a negative `timeout` will be interpreted as **block forever**.

## Known Restrictions

### 1. Abbreviated IPv4 address parsing

- Due to the fact that Rust uses `inet_pton()` function internally for parsing `IPv4` strings to `IpAddr` struct, **only** (`a.b.c.d`) format is supported.
  - For example: 
    - `“1.1.1.1"` is ok.
    - `“1.0.0.1”` is ok.  
    - `“1.1"` will **fail**, although it is equivalent to `“1.0.0.1”`.
    - `“1.0.1"` will **fail**, although it is equivalent to `“1.0.0.1”`.
    - `“127.0.0.1”` is ok.  
    - `“127.0.1”` is ok because I handled this **specifically**.
    - `“127.1”` is ok because I handled this **specifically**.
- This can be solved by manual parsing, but doing so is not necessary.

### 2. Argument Overwriting

- Duplicated arguments passed to the program will **overwrite** the previous ones.
  - For example:
    - `ping-cli -c 3 -c 2 c 1 -c 0` is equivalent to `ping-cli -c 0`.
- Also note that if an invalid argument is passed as duplicated arguments, the program will also **exit**.
  - For example:
    - `ping-cli -c -1 -c 1` will not work because `-1` is invalid.
