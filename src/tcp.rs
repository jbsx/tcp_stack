pub(crate) struct State{

}

impl Default for State{
    fn default() -> Self {
        State { 
            
        }
    }
}

impl State{
    pub fn on_packet<'a>(
        &mut self,
        tcp_header: etherparse::TcpHeaderSlice<'a>,
        ip_header: etherparse::Ipv4HeaderSlice<'a>,
        buff: &'a [u8]
    ){
        eprintln!("{:?} : {:?} -> {:?} : {:?}, {:?} bytes", ip_header.source_addr(), tcp_header.source_port(), ip_header.destination_addr(), tcp_header.destination_port(), buff.len());
    }
}

