use chrono::{Datelike, Timelike, Utc};
use futures::StreamExt;
//use std::io::{self, Write};
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

        // Wait for connections. On receiving a connection spawn it off into a new "green thread"
        // and run the handler.
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

async fn handler(mut socket: TcpStream) {
    let t = Duration::new(2, 0);

    let buf = log_entry(&format!(
        "Accepted connection from {:?}",
        socket.peer_addr(),
    ));

    let mut buf = [0; 1024];
    match socket.read(&mut buf).await {
        // socket closed
        Ok(n) => {
            io::stdout().write_all(&buf[0..n]).await;
        }
        Err(e) => {
            println!("failed to read from socket; err = {:?}", e);
            return;
        }
    };
    delay_for(t).await;

    log_entry(&format!(
        "Finished connection from {:?}",
        socket.peer_addr()
    ))
}

fn log_entry(msg: &str) {
    println!("{} -- {}", fmt_date(), msg)
}

fn fmt_date() -> String {
    let now = Utc::now();

    now.to_rfc3339()
}
