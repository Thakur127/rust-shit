use crate::cache::Cache;
use crate::config::ProxyServerConfig;
use log;
use std::cell::RefCell;
use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpStream};
use std::rc::Rc;

pub fn handle_connection<C: Cache>(
    mut stream: TcpStream,
    remote_server: Option<&mut TcpStream>,
    cache: Rc<RefCell<C>>,
    proxy_config: Option<&ProxyServerConfig>,
) {
    log::info!(
        "Handling connections for client {}",
        stream.peer_addr().unwrap_or_else(|err| {
            log::error!("Failed to find ip address of client");
            log::error!("Error: {err}");
            SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 0), 00000))
        })
    );

    let mut buffer = [0; 1024];
    let mut remote_buffer = [0; 1024];

    let bytes_read = stream.read(&mut buffer).unwrap_or_else(|err| {
        log::error!("Failed to read from client");
        log::error!("Error: {err}");
        0
    });
    if bytes_read == 0 {
        log::info!("Client {} disconnected", stream.peer_addr().unwrap());
        return;
    }

    let request = &buffer[..bytes_read];
    let mut req_headers = [httparse::EMPTY_HEADER; 16];

    // Parse the request
    let parsed_request = parse_request(&request, &mut req_headers);
    println!("{}", parsed_request.path.unwrap());

    // Send the request to the remote server
    if let Some(remote_server) = remote_server {
        // serve request from cache if exists
        let key = make_key_from_path(&parsed_request.path.unwrap());
        if let Some(response) = cache.borrow_mut().get(&key) {
            log::info!("Serving request from cache");

            stream.write(response.as_bytes()).unwrap();
            return;
        }

        // send request to remote server
        remote_server.write(request).unwrap_or_else(|err| {
            log::error!("{}", err);
            0
        });

        // Read the response from the remote server
        let bytes_read = remote_server.read(&mut remote_buffer).unwrap();
        if bytes_read == 0 {
            return;
        }

        // cache the response if pathname is in proxy_config
        if let Some(proxy_config) = proxy_config {
            let pathname = parsed_request
                .path
                .unwrap()
                .split('?')
                .collect::<Vec<&str>>()[0];

            // find ttl for pathname
            let cache_path = proxy_config
                .cache
                .as_ref()
                .unwrap()
                .paths
                .as_ref()
                .unwrap()
                .iter()
                .find(|path| path.pathname == pathname);

            if let Some(cache_path) = cache_path {
                let key = make_key_from_path(&parsed_request.path.unwrap());
                cache.borrow_mut().set(
                    &key,
                    &String::from_utf8(remote_buffer[..bytes_read].to_vec()).unwrap(),
                    Some(cache_path.ttl),
                );
            }
        }

        // Send the response to the client
        stream.write(&remote_buffer[..bytes_read]).unwrap();
        return;
    }

    // Send a response if no remote server is provided
    stream
        .write(b"HTTP/1.1 200 OK\n\nHello from the junk server\n")
        .unwrap();
}

fn parse_request<'b, 'h>(
    request: &'b [u8],
    headers: &'h mut [httparse::Header<'h>],
) -> httparse::Request<'b, 'h>
where
    'b: 'h, // relates the lifetime of the request to the lifetime of the headers
{
    let mut req = httparse::Request::new(headers);
    req.parse(request).unwrap();
    req
}

fn make_key_from_path(path: &str) -> String {
    let mut key = String::new();

    // Split path into parts (before and after '?')
    let parts: Vec<&str> = path.split('?').collect();
    key.push_str(parts[0]); // Add the path to the key

    // If there are query parameters, process them
    if parts.len() > 1 {
        let query_string = parts[1];

        // Parse query parameters into a vector of tuples
        let mut query_params: Vec<(String, String)> = query_string
            .split('&')
            .filter_map(|param| {
                let mut split = param.splitn(2, '=');
                if let (Some(key), Some(value)) = (split.next(), split.next()) {
                    Some((key.to_string(), value.to_string()))
                } else {
                    None
                }
            })
            .collect();

        // Sort query parameters by key
        query_params.sort_by(|a, b| a.0.cmp(&b.0));

        // Add the sorted query parameters to the key
        for (key_, value) in query_params {
            key.push_str(&format!(":{}={}", key_, value));
        }
    }

    key
}
