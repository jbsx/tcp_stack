use std::collections::HashMap;
use std::io;
use std::net::Ipv4Addr;

mod tcp;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct AddrQuad{
    src: Ipv4Addr,
    src_port: u16,
    dst: Ipv4Addr,
    dst_port: u16
}

fn main() -> io::Result<()>{
    let mut connections: HashMap<AddrQuad, tcp::State> = Default::default();
    let nic = tun_tap::Iface::new("tun", tun_tap::Mode::Tun)?;
    let mut buff = [0u8; 1504];
    loop{
        let recv_len = nic.recv(&mut buff).unwrap();
        if u16::from_be_bytes([buff[2], buff[3]]) != 0x0800 {continue}
        match etherparse::Ipv4HeaderSlice::from_slice(&buff[4..recv_len]){
            Ok(header)=>{
                eprintln!("Recieved {} bytes of IPV4 packet from {:?} -> {:?}", header.payload_len(), header.source_addr(), header.destination_addr());
                match etherparse::TcpHeaderSlice::from_slice(&buff[4+header.slice().len()..]){
                    Ok(res)=>{
                        eprintln!("{:?}", &res);
                        connections.entry( AddrQuad{
                            src: header.source_addr(),
                            src_port: res.source_port(),
                            dst: header.destination_addr(),
                            dst_port: res.destination_port()
                        }).or_default();
                    },
                    Err(e)=>{
                        eprintln!("{}", e);
                    }
                }
            },
            Err(e)=>{
                eprintln!("{}", e)
            }
        }
    }
}
