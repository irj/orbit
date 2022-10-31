use nano_leb128::ULEB128;

pub struct OsuWriter {
    data: Vec<u8>,
}

impl OsuWriter {
    pub fn new() -> Self {
        Self::from_vec(vec![])
    }

    pub fn from_vec(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn write_byte(&mut self, val: u8) {
        self.data.extend_from_slice(&val.to_le_bytes());
    }

    pub fn write_short(&mut self, val: u16) {
        self.data.extend_from_slice(&val.to_le_bytes());
    }

    pub fn write_int(&mut self, val: u32) {
        self.data.extend_from_slice(&val.to_le_bytes());
    }

    pub fn write_long(&mut self, val: u64) {
        self.data.extend_from_slice(&val.to_le_bytes());
    }

    pub fn write_uleb(&mut self, val: ULEB128) {
        let mut bytes: [u8; 512] = [0; 512];
        let _ = val.write_into(&mut bytes);

        self.data.extend_from_slice(&bytes);
    }

    pub fn write_single(&mut self, val: f32) {
        self.data.extend_from_slice(&val.to_le_bytes())
    }

    pub fn write_double(&mut self, val: f64) {
        self.data.extend_from_slice(&val.to_le_bytes())
    }

    pub fn write_boolean(&mut self, val: bool) {
        self.write_byte(if val { 1 } else { 0 })
    }

    pub fn write_string(&mut self, val: Option<String>) {
        match val {
            None => self.data.push(0x00),
            Some(str) => {
                self.data.push(0x0b);

                let bytes = str.as_bytes();
                let len = bytes.len() as u64;

                self.write_uleb(len.into());
                self.data.extend_from_slice(bytes);
            }
        }
    }
}
