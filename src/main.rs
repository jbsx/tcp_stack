use std::collections::HashMap;
use std::io;
use std::net::Ipv4Addr;

mod tcp;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct AddrQuad {
    src: Ipv4Addr,
    src_port: u16,
    dst: Ipv4Addr,
    dst_port: u16,
}

fn main() -> io::Result<()> {
    let mut connections: HashMap<AddrQuad, tcp::Connection> = Default::default();
    let mut nic = tun_tap::Iface::new("tun", tun_tap::Mode::Tun)?;
    let mut buff = [0u8; 1504];
    loop {
        let recv_len = nic.recv(&mut buff).unwrap();
        if u16::from_be_bytes([buff[2], buff[3]]) != 0x0800 {
            continue;
        }
        match etherparse::Ipv4HeaderSlice::from_slice(&buff[4..recv_len]) {
            Ok(ip_header) => {
                eprintln!(
                    "Recieved {} bytes of IPV4 packet from {:?} -> {:?}",
                    ip_header.payload_len(),
                    ip_header.source_addr(),
                    ip_header.destination_addr()
                );
                match etherparse::TcpHeaderSlice::from_slice(&buff[4 + ip_header.slice().len()..recv_len]) {
                    Ok(tcp_header) => {
                        eprintln!("{:?}", tcp_header.destination_port());
                        let data_len = ip_header.slice().len() + tcp_header.slice().len() + 4;
                        connections.entry(AddrQuad {
                                src: ip_header.source_addr(),
                                src_port: tcp_header.source_port(),
                                dst: ip_header.destination_addr(),
                                dst_port: tcp_header.destination_port(),
                            })
                            .accept(&mut nic, tcp_header, ip_header, &buff[data_len..recv_len])?;
                    }
                    Err(e) => {
                        eprintln!("ignoring weird packet : {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("{}", e)
            }
        }
    }
}

