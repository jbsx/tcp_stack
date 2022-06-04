use std::io;
fn main() -> io::Result<()>{    
    let nic = tun_tap::Iface::new("tun", tun_tap::Mode::Tun)?;
    let mut buff = [0u8; 1504];
    loop{
        let recv_len = nic.recv(&mut buff).unwrap();
        if u16::from_be_bytes([buff[2], buff[3]]) != 0x0800 {continue}
        match etherparse::Ipv4HeaderSlice::from_slice(&buff[4..recv_len]){
            Ok(header)=>{
                eprintln!("Recieved {} bytes of IPV4 packet from {:?}", header.payload_len(), header.source_addr());
                match etherparse::TcpHeaderSlice::from_slice(&buff[4+header.slice().len()..]){
                    Ok(res)=>{
                        eprintln!("{:?}aslfdkjaslkdjlj", &res);
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
