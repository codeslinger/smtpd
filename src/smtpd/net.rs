// Copyright (c) 2014 Toby DiPasquale <toby@cbcg.net>
use std::io::{TcpListener, Acceptor, Listener, TcpStream};
use std::thread::Thread;

pub fn main_loop(config: ::config::Config) {
    let listener = match TcpListener::bind(config.listen_address.as_slice()) {
        Ok(listener) => { listener }
        Err(e) => { error!("failed to start TCP listener on {}: {}", config.listen_address, e); return }
    };
    let mut acceptor = match listener.listen() {
        Ok(acceptor) => { acceptor }
        Err(e) => { error!("failed to bind to {}: {}", config.listen_address, e); return }
    };
    for stream in acceptor.incoming() {
        match stream {
            Ok(stream) => { Thread::spawn(move|| { handle_connection(stream) }).detach() }
            Err(e) => { error!("failed to accept connection: {}", e) }
        }
    }
    drop(acceptor);
}

fn handle_connection(mut stream: TcpStream) {
    info!("client connect from {}", stream.peer_name().unwrap());
}

