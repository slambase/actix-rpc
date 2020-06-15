use std::{io, net};
use actix_server::{Server};
use actix_service::{fn_service};
use actix_rt::net::TcpStream;
use actix_codec::{Framed, BytesCodec};
use socket2::{Socket, Domain, Type, Protocol};
use bytes::Bytes;
use futures_util::SinkExt;

fn main() -> io::Result<()> {

    let addr: net::SocketAddr = "0.0.0.0:8212".parse().unwrap();

    let sys = actix_rt::System::new("server");
    let srv: Server = Server::build()
        .backlog(100)
        .disable_signals()
        .bind("test", addr, move || {
            println!("outer service");
            fn_service(|io: TcpStream| async move {
                println!("inner service");
                let mut f = Framed::new(io, BytesCodec);
                f.send(Bytes::from_static(b"test")).await.unwrap();
                Ok::<_, ()>(())
            })
        })
        .unwrap()
        .start();

    let _ = sys.run();

    Ok(())
}