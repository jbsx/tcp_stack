use std::io;
fn main() -> io::Result<()>{    
    let nic = tun_tap::Iface::new("tun", tun_tap::Mode::Tun)?;
    let mut buff = [0u8; 1504];
    loop{
        let recv_len = nic.recv(&mut buff).unwrap();
        let flags = u16::from_be_bytes([buff[0], buff[1]]);
        let protocol = u16::from_be_bytes([buff[2], buff[3]]);
        eprintln!("recieved {} bytes of protocol {:x?} with flags: {:x?}: {:x?}", recv_len, protocol, flags, &buff[4..recv_len-4]);
    }
}
