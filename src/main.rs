use std::net::{TcpListener, IpAddr, Ipv4Addr, SocketAddr};

use own_server::thread_pool::ThreadPool;
use own_server::http::handle_connection;


fn main() {

    const HOST: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    const PORT: u16 = 8000;
    let sock_addr = SocketAddr::from((HOST, PORT));
    let server = match TcpListener::bind(sock_addr) {
        Ok(value) => {
            println!("Successfully bound to address: {}", sock_addr);
            value
        },
        Err(err) => {
            panic!("Failed to bind to address: {}\nerror: {}", sock_addr, err);
        }
    };
    let pool = ThreadPool::new(4);

    for stream in server.incoming() {
        let stream = match stream {
            Ok(value) => {
                println!("Connection Accepted");
                value
            },
            Err(err) => {
                println!("Failed to accept connection: {}", err);
                continue;
            }
        };
        pool.execute(|| handle_connection(stream));
    }
}

