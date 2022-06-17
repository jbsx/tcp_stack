pub(crate) enum State{
    Closed,
    Listen,
    SynRcvd,
    Estab
}

impl Default for State{
    fn default() -> Self {
        State::Closed
    }
}

impl State{
    pub fn on_packet<'a>(
        &mut self,
        nic: &mut tun_tap::Iface
        tcp_header: etherparse::TcpHeaderSlice<'a>,
        ip_header: etherparse::Ipv4HeaderSlice<'a>,
        buff: &'a [u8]
    ){
        match *self{
            State::Closed => {
                return
            },
            State::Listen => {
                let mut buff = [0u8; 1504];
                if !tcp_header.syn(){return}
                let synack = etherparse::TcpHeader::new(
                    tcp_header.destination_port(),
                    tcp_header.source_port(),
                    0,
                    0
                );
                synack.syn = true;
                synack.ack = true;
                let mut ip = etherparse::Ipv4Header::new(
                    synack.slice().len(),
                    64,
                    etherparse::IpNumber::Tcp,
                    ip_header.source(),
                    ip_header.destination()
                );
                let mut unwritten = &mut buff[..];
                ip.write(unwritten);
                synack.write(unwritten);
                nic.write(unwritten);
            }
        }
        eprintln!("{:?} : {:?} -> {:?} : {:?}, {:?} bytes", 
            ip_header.source_addr(), 
            tcp_header.source_port(), 
            ip_header.destination_addr(), 
            tcp_header.destination_port(), 
            buff.len());
    }
}

