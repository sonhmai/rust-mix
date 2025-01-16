use std::collections::HashMap;
use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{channel, Sender};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

/// Use tokio::select! for running 2 tasks concurrently in a continuous loop.
/// - 1st one receives messages from clients and broadcast them.
/// - 2nd sends messages received by server to client.
async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
    clients: Arc<Mutex<HashMap<SocketAddr, Sender<String>>>>,
) -> Result<(), Box<dyn Error + Send + Sync>> {

    ws_stream
        .send(Message::text("Welcome to chat! Type a message".to_string()))
        .await?;
    let mut bcast_rx = bcast_tx.subscribe();

    loop {
        tokio::select! {
            // futures_util::stream::StreamExt::next() for async reading msgs from ws stream
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            println!("From client {addr:?} {text:?}");
                            let clients = clients.lock().unwrap();
                            for (client_addr, client_tx) in clients.iter() {
                                if *client_addr != addr {
                                    client_tx.send(text.into()).unwrap(); // Send to all except sender
                                }
                            }
                        }
                    }
                    Some(Err(err)) => return Err(err.into()),
                    None => return Ok(()),
                }
            }
            msg = bcast_rx.recv() => {
                // futures_util::sink::SinkExt::send for async send msgs on ws stream
                ws_stream.send(Message::text(msg?)).await?;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);
    // Arc instead of Rc as multiple tasks can mutate this in different threads.
    // Rc is only for single-threaded env, Arc provides atomic ref count update.
    let clients = Arc::new(Mutex::new(HashMap::new()));
    let listener = TcpListener::bind("127.0.0.1:2000").await?;
    println!("server listening on port 2000");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        let bcast_tx = bcast_tx.clone();
        let clients = Arc::clone(&clients);
        tokio::spawn(async move {
            clients.lock().unwrap().insert(addr, bcast_tx.clone());
            // Wrap the raw TCP stream into a websocket.
            let ws_stream = ServerBuilder::new().accept(socket).await?;
            handle_connection(addr, ws_stream, bcast_tx, clients).await
        });
    }
}