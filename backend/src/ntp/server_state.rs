use super::{NtpTimestamp, NtpFracValue};

#[derive(Copy, Clone)]
pub struct ServerState {
    pub leap: u8,
    pub stratum: u8,
    pub precision: i8,
    pub ref_id: u32,
    pub ref_ts: NtpTimestamp,
    pub dispersion: NtpFracValue,
    pub delay: NtpFracValue,
}