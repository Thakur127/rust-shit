use backdoor::cache::{Cache, InMemoryCache};
use backdoor::config::{CacheConfig, CacheConfigPathOption, ProxyServerConfig};
use backdoor::connection::handle_connection;

use env_logger;
use log;
use std::cell::RefCell;
use std::net::{TcpListener, TcpStream};
use std::rc::Rc;

fn main() {
    // on terminal execute, "export RUST_LOG=trace" to see all logs
    env_logger::init();

    let in_memory_cache = Rc::new(RefCell::new(InMemoryCache::new()));

    let config = ProxyServerConfig {
        host: Some("127.0.0.1".to_string()),
        port: Some("3452".to_string()),
        target_hosts: Some(vec![
            "127.0.0.1:3451".to_string(),
            "127.0.0.1:3453".to_string(),
        ]),
        cache: Some(CacheConfig {
            paths: Some(vec![CacheConfigPathOption {
                pathname: "/".to_string(),
                ttl: 60,
            }]),
        }),
    };

    let server_address = format!(
        "{}:{}",
        config.host.as_ref().unwrap(),
        config.port.as_ref().unwrap()
    );

    // create a TCP listener
    let listener = TcpListener::bind(&server_address).unwrap_or_else(|err| {
        log::error!("Error creating TCPListener: {}", err);
        std::process::exit(1);
    });

    log::info!("Server Started on {}", server_address);
    log::info!("Listening for connections...");

    // create remote connections beforehand so, there will be no overhead
    // of creating connection for each request
    let mut remote_connections: Vec<Option<TcpStream>> = Vec::new();

    if let Some(target_hosts) = config.target_hosts.as_ref() {
        for target_host in target_hosts {
            match TcpStream::connect(target_host) {
                Ok(stream) => {
                    log::info!(
                        "Connection established with {}",
                        stream.peer_addr().unwrap()
                    );
                    remote_connections.push(Some(stream));
                }
                Err(_) => {
                    log::error!("Connection failed with: {}", target_host);
                }
            }
        }
    }

    let mut target_conn_idx = 0;

    // distribute requests to remote servers using round robin algorithm
    for stream in listener.incoming() {
        if remote_connections.len() == 0 {
            handle_connection(stream.unwrap(), None, Rc::clone(&in_memory_cache), None);
        } else {
            handle_connection(
                stream.unwrap(),
                remote_connections[target_conn_idx].as_mut(),
                Rc::clone(&in_memory_cache),
                Some(&config),
            );
            target_conn_idx = (target_conn_idx + 1) % remote_connections.len();
        }
    }
}
