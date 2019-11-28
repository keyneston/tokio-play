use futures::StreamExt;
use std::time::{Duration, SystemTime};
use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio::time::delay_for;

#[tokio::main]
async fn main() {
    println!("Starting server...");

    let addr = "127.0.0.1:6142";
    let mut listener = TcpListener::bind(addr).await.unwrap();

    let server = async move {
        let mut incoming = listener.incoming();

        while let Some(socket_res) = incoming.next().await {
            match socket_res {
                Ok(socket) => {
                    tokio::spawn(async move {
                        handler(socket).await;
                    });
                }
                Err(err) => {
                    println!("accept error = {:?}", err);
                }
            }
        }
    };

    println!("Server is running on {0}", addr);
    server.await;
}

async fn handler(socket: TcpStream) {
    let t = Duration::new(2, 0);

    println!(
        "Accepted connection from {:?} at {1:?}",
        socket.peer_addr(),
        SystemTime::now(),
    );
    delay_for(t).await;
    println!(
        "Finished connection from {:?} at {1:?}",
        socket.peer_addr(),
        SystemTime::now(),
    );
}
