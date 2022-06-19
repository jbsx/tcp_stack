use std::io;
use std::io::prelude::*;
pub(crate) enum State{
    Closed,
    Listen,
    // SynRcvd,
    // Estab
}

impl Default for State{
    fn default() -> Self {
        State::Closed
    }
}

impl State{
    pub fn on_packet<'a> (
        &mut self,
        nic: &mut tun_tap::Iface,
        tcp_header: etherparse::TcpHeaderSlice<'a>,
        ip_header: etherparse::Ipv4HeaderSlice<'a>,
        _data: &'a [u8]
    ) -> io::Result<usize> {
        match *self{
            State::Closed => {
                return Ok(0)
            },
            State::Listen => {
                let mut buf = [0u8; 1500];
                if !tcp_header.syn(){return Ok(0)}
                let synack = etherparse::TcpHeader::new(
                    tcp_header.destination_port(),
                    tcp_header.source_port(),
                    unimplemented!(),
                    unimplemented!());
                synack.syn = true;
                synack.ack = true;
                let mut ip = etherparse::Ipv4Header::new(
                    synack.header_len(),
                    64,
                    etherparse::IpNumber::Tcp,
                    ip_header.source(),
                    ip_header.destination()
                );
                let unwritten = {
                    let mut unwritten = &mut buf[..];
                    ip.write(&mut unwritten).unwrap();
                    synack.write(&mut unwritten)?;
                    unwritten.len()
                };
                eprintln!("{:?}", &buf[..unwritten]);
                nic.send(&buf[..unwritten])
            },
        }
    }
}

