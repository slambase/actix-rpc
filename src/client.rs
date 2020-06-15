use std::{io, net};
use std::io::Read;

fn main() -> io::Result<()> {

    let addr: net::SocketAddr = "0.0.0.0:8212".parse().unwrap();

    let mut buf = [1u8; 4];
    let mut conn = net::TcpStream::connect(addr).unwrap();
    let _ = conn.read_exact(&mut buf);
    println!("{:?}", String::from_utf8(buf.to_vec()));
    assert_eq!(buf, b"test"[..]);

    Ok(())

}