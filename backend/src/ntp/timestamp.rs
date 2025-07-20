use std::time::SystemTime;

use byteorder::{BigEndian, ByteOrder};
use rand::random;
use serde::{Deserialize, Serialize};

 #[derive(Debug, Copy, Clone)]
 #[derive( Serialize, Deserialize)]
 pub struct Timestamp {
 pub ts: u64,
}

impl Timestamp {
    pub fn new(ts:u64)->Self{
        Self{ ts }
    }
    pub fn now() -> Timestamp {
        let now = SystemTime::now();
        let dur = now.duration_since(std::time::UNIX_EPOCH).unwrap();
        let secs = dur.as_secs() + 2208988800; // 1900 epoch
        let nanos = dur.subsec_nanos();

        Timestamp{ts: (secs << 32) + (nanos as f64 * 4.294967296) as u64}
    }

    pub fn zero() -> Timestamp {
        Timestamp{ts: 0}
    }

    pub fn random() -> Timestamp {
        Timestamp{ts: random()}
    }

    pub fn diff_to_sec(&self, ts: &Timestamp) -> f64 {
        (self.ts.wrapping_sub(ts.ts)) as i64 as f64 / 4294967296.0
    }

    pub fn read(buf: &[u8]) -> Timestamp {
        Timestamp{ts: BigEndian::read_u64(buf)}
    }

    pub fn write(&self, buf: &mut [u8]) {
        BigEndian::write_u64(buf, self.ts);
    }
}

impl PartialEq for Timestamp {
    fn eq(&self, other: &Timestamp) -> bool {
        self.ts == other.ts
    }
}