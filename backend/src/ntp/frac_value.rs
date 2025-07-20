use byteorder::{BigEndian, ByteOrder};

#[derive(Debug, Copy, Clone)]
pub struct FracValue {
    val: u32,
}

impl FracValue {
    pub fn read(buf: &[u8]) -> FracValue {
        FracValue{val: BigEndian::read_u32(buf)}
    }

    pub fn write(&self, buf: &mut [u8]) {
        BigEndian::write_u32(buf, self.val);
    }

    pub fn zero() -> FracValue {
        FracValue{val: 0}
    }

    pub fn increment(&mut self) {
        self.val += 1;
    }
}