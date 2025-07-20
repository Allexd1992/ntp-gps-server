use std::io;
use std::io::{Error, ErrorKind};

use byteorder::{BigEndian, ByteOrder};

use super::{NtpPacket, NtpServerState};

use super::{NtpTimestamp, NtpFracValue};
use std::net::SocketAddr;
use tokio::{
    net::UdpSocket,
    sync::{mpsc::UnboundedReceiver, Mutex},
};

#[derive(Debug)]
pub struct Packet {
   pub remote_addr: SocketAddr,
    pub local_ts: NtpTimestamp,

    pub leap: u8,
    pub version: u8,
    pub mode: u8,
    pub stratum: u8,
    pub poll: i8,
    pub precision: i8,
    pub delay: NtpFracValue,
    pub dispersion: NtpFracValue,
    pub ref_id: u32,
    pub ref_ts: NtpTimestamp,
    pub orig_ts: NtpTimestamp,
    pub rx_ts: NtpTimestamp,
    pub tx_ts: NtpTimestamp,
}

impl Packet {
   pub async  fn receive(socket: &UdpSocket) -> io::Result<NtpPacket> {
        let mut buf = [0; 1024];

        let (len, addr) = socket.recv_from(&mut buf).await.unwrap();

        let local_ts = NtpTimestamp::now();

        if len < 48 {
            return Err(Error::new(ErrorKind::UnexpectedEof, "Packet too short"));
        }

        let leap = buf[0] >> 6;
        let version = (buf[0] >> 3) & 0x7;
        let mode = buf[0] & 0x7;

        if version < 1 || version > 4 {
            return Err(Error::new(ErrorKind::Other, "Unsupported version"));
        }

        Ok(NtpPacket{
            remote_addr: addr,
            local_ts: local_ts,
            leap: leap,
            version: version,
            mode: mode,
            stratum: buf[1],
            poll: buf[2] as i8,
            precision: buf[3] as i8,
            delay: NtpFracValue::read(&buf[4..8]),
            dispersion: NtpFracValue::read(&buf[8..12]),
            ref_id: BigEndian::read_u32(&buf[12..16]),
            ref_ts: NtpTimestamp::read(&buf[16..24]),
            orig_ts: NtpTimestamp::read(&buf[24..32]),
            rx_ts: NtpTimestamp::read(&buf[32..40]),
            tx_ts: NtpTimestamp::read(&buf[40..48]),
        })
    }

    pub async fn send(&self, socket: &UdpSocket) -> io::Result<usize> {
        let mut buf = [0; 48];

        buf[0] = self.leap << 6 | self.version << 3 | self.mode;
        buf[1] = self.stratum;
        buf[2] = self.poll as u8;
        buf[3] = self.precision as u8;
        self.delay.write(&mut buf[4..8]);
        self.dispersion.write(&mut buf[8..12]);
        BigEndian::write_u32(&mut buf[12..16], self.ref_id);
        self.ref_ts.write(&mut buf[16..24]);
        self.orig_ts.write(&mut buf[24..32]);
        self.rx_ts.write(&mut buf[32..40]);
        self.tx_ts.write(&mut buf[40..48]);

        socket.send_to(&buf, self.remote_addr).await
    }

    pub fn is_request(&self) -> bool {
        self.mode == 1 || self.mode == 3 ||
            (self.mode == 0 && self.version == 1 && self.remote_addr.port() != 123)
    }

    pub fn make_response(&self, state: &NtpServerState) -> Option<NtpPacket> {
        if !self.is_request() {
            return None;
        }

        Some(NtpPacket{
            remote_addr: self.remote_addr,
            local_ts: NtpTimestamp::zero(),
            leap: state.leap,
            version: self.version,
            mode: if self.mode == 1 { 2 } else { 4 },
            stratum: state.stratum,
            poll: self.poll,
            precision: state.precision,
            delay: state.delay,
            dispersion: state.dispersion,
            ref_id: state.ref_id,
            ref_ts: state.ref_ts,
            orig_ts: self.tx_ts,
            rx_ts: self.local_ts,
            tx_ts: NtpTimestamp::now(),
        })
    }

    pub async fn new_request(remote_addr: SocketAddr) -> NtpPacket {
        NtpPacket{
            remote_addr: remote_addr,
            local_ts: NtpTimestamp::now(),
            leap: 0,
            version: 4,
            mode: 3,
            stratum: 0,
            poll: 0,
            precision: 0,
            delay: NtpFracValue::zero(),
            dispersion: NtpFracValue::zero(),
            ref_id: 0,
            ref_ts: NtpTimestamp::zero(),
            orig_ts: NtpTimestamp::zero(),
            rx_ts: NtpTimestamp::zero(),
            tx_ts: NtpTimestamp::random(),
        }
    }

   pub  fn is_valid_response(&self, request: &NtpPacket) -> bool {
        self.remote_addr == request.remote_addr &&
            self.mode == request.mode + 1 &&
            self.orig_ts == request.tx_ts
    }

   pub fn get_server_state(&self) -> NtpServerState {
        NtpServerState{
            leap: self.leap,
            stratum: self.stratum,
            precision: self.precision,
            ref_id: self.ref_id,
            ref_ts: self.ref_ts,
            dispersion: self.dispersion,
            delay: self.delay,
        }
    }


}