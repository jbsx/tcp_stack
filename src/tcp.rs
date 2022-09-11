use std::io;
enum State {
    Closed,
    Listen,
    SynRcvd,
    Estab,
}

pub struct SendSequence {
    una: u32,
    nxt: u32,
    wnd: u16,
    up: bool,
    wl1: usize,
    wl2: usize,
    iss: u32,
}

pub struct RecvSequence {
    nxt: u32,
    wnd: u16,
    up: bool,
    irs: u32,
}

pub struct Connection {
    state: State,
    send: SendSequence,
    recv: RecvSequence,
}

impl Connection {
    pub fn accept<'a>(
        &mut self,
        nic: &mut tun_tap::Iface,
        tcp_header: etherparse::TcpHeaderSlice<'a>,
        ip_header: etherparse::Ipv4HeaderSlice<'a>,
        _data: &'a [u8],
    ) -> io::Result<Option<Self>> {
        let mut buf = [0u8; 1500];
        if !tcp_header.syn() {
            return Ok(None);
        }
        let c = Connection {
            state: State::SynRcvd,
            send: SendSequence {
                iss: 0,
                una: self.send.iss,
                nxt: self.send.una + 1,
                wnd: 10,
                up: false,
                wl1: 0,
                wl2: 0,
            },
            recv: RecvSequence {
                wnd: tcp_header.window_size(),
                irs: tcp_header.sequence_number(),
                nxt: tcp_header.sequence_number() + 1,
                up: false,
            },
        };
        let mut synack = etherparse::TcpHeader::new(
            tcp_header.destination_port(),
            tcp_header.source_port(),
            c.send.iss,
            c.send.wnd,
        );
        synack.acknowledgment_number = tcp_header.sequence_number() + 1;
        synack.syn = true;
        synack.ack = true;
        let mut ip = etherparse::Ipv4Header::new(
            synack.header_len(),
            64,
            etherparse::IpNumber::Tcp,
            ip_header.source(),
            ip_header.destination(),
        );
        let unwritten = {
            let mut unwritten = &mut buf[..];
            ip.write(&mut unwritten).unwrap();
            synack.write(&mut unwritten)?;
            unwritten.len()
        };
        nic.send(&buf[..unwritten])?;
        Ok(Some(c))
    }
    pub fn on_packet<'a>() -> io::Result<Option<Connection>> {
        unimplemented!();
    }
}
