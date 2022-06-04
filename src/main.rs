use std::io;
fn main() -> io::Result<()>{    
    let nic = tun_tap::Iface::new("tun", tun_tap::Mode::Tun)?;
    let mut buff = [0u8; 1504];
    loop{
        let recv_len = nic.recv(&mut buff).unwrap();
        if u16::from_be_bytes([buff[2], buff[3]]) != 0x0800 {continue}
        match etherparse::Ipv4Header::from_slice(&buff[4..recv_len]){
            Ok((header, _))=>{
                eprintln!("Recieved {} bytes of IPV4 packet from {:?}", header.total_len(), header.source)
            },
            Err(e)=>{
                eprintln!("{}", e)
            }
        }
    }
}
