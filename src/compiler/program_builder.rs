#[derive(Default, Clone, PartialEq, Eq)]
pub struct ProgramBuilder {
    bytes: Vec<u8>,
}

impl ProgramBuilder {
    pub fn push_u8(&mut self, byte: u8) {
        self.bytes.push(byte);
    }

    pub fn push_u32(&mut self, value: u32) {
        self.bytes.extend(value.to_le_bytes());
    }

    pub fn push_char(&mut self, value: char) {
        self.push_u32(value as u32);
    }

    pub fn into_bytes(mut self) -> Box<[u8]> {
        self.bytes.shrink_to_fit();
        self.bytes.into()
    }
}
