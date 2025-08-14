use::crate::{
    common_ports::MOST_COMMON_PORTS_10,
    model::{Port, Subdomain },
};
use::std::net::{SocketAddr, ToSocketAddrs};
use::std::{net::TcpStream, time::Duration};
use::rayon::prelude::*;

pub fn scan_ports(mut subdomain: Subdomain) -> Subdomain {
    subdomain.open_ports = MOST_COMMON_PORTS_10
        .into_par_iter()
        .map(|port| scan_port(&subdomain.domain, *port))
        .filter(|port| port.is_open)
        .collect();
    subdomain
}
pub fn scan_port(hostname: &str, port: u16) -> Port {
    let timeout = Duration::from_secs(3);
    let socket_address: Vec<SocketAddr> = format!("{}:{}",hostname, port)
        .to_socket_addrs()
        .expect("port scanner: Creating socket address")
        .collect();

    if socket_address.len == 0 {
        return Port {
            port: port,
            is_open: false,
        };
    }
    let is_open = if let Ok(_) = TcpStream::connect_timeout(&socket_address[0],timeout) {
        true
    } else {
        false
    };
    
    Port {
        port: port,
        is_open,
    }
}
