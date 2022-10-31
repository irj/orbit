use nano_leb128::ULEB128;

pub struct OsuReader {
    data: Vec<u8>,
    pointer: usize,
}

pub enum ParseError {
    InvalidData,
    EndOfFile,
}

impl OsuReader {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self::from_vec(bytes.to_vec())
    }

    pub fn from_vec(data: Vec<u8>) -> Self {
        Self { data, pointer: 0 }
    }

    fn check(&self, count: usize) -> Result<(), ParseError> {
        if self.pointer + count > self.data.len() {
            return Err(ParseError::EndOfFile);
        }

        Ok(())
    }

    pub fn read_byte(&mut self) -> Result<u8, ParseError> {
        self.check(1)?;

        let byte = self.data[self.pointer];
        self.pointer += 1;

        Ok(u8::from_le_bytes([byte]))
    }

    pub fn read_short(&mut self) -> Result<u16, ParseError> {
        self.check(2)?;

        let bytes = &self.data[self.pointer..self.pointer + 2];
        let val = u16::from_le_bytes(bytes.try_into().map_err(|_| ParseError::InvalidData)?);

        self.pointer += 2;
        Ok(val)
    }

    pub fn read_int(&mut self) -> Result<u32, ParseError> {
        self.check(4)?;

        let bytes = &self.data[self.pointer..self.pointer + 4];
        let val = u32::from_le_bytes(bytes.try_into().map_err(|_| ParseError::InvalidData)?);

        self.pointer += 4;
        Ok(val)
    }

    pub fn read_long(&mut self) -> Result<u64, ParseError> {
        self.check(8)?;

        let bytes = &self.data[self.pointer..self.pointer + 8];
        let val = u64::from_le_bytes(bytes.try_into().map_err(|_| ParseError::InvalidData)?);

        self.pointer += 8;
        Ok(val)
    }

    pub fn read_uleb(&mut self) -> Result<ULEB128, ParseError> {
        self.check(1)?;

        let (uleb, size) =
            ULEB128::read_from(&self.data[self.pointer..]).map_err(|_| ParseError::InvalidData)?;

        self.pointer += size;
        Ok(uleb)
    }

    pub fn read_single(&mut self) -> Result<f32, ParseError> {
        self.check(4)?;

        let bytes = &self.data[self.pointer..self.pointer + 4];
        let val = f32::from_le_bytes(bytes.try_into().map_err(|_| ParseError::InvalidData)?);

        self.pointer += 4;
        Ok(val)
    }

    pub fn read_double(&mut self) -> Result<f64, ParseError> {
        self.check(8)?;

        let bytes = &self.data[self.pointer..self.pointer + 8];
        let val = f64::from_le_bytes(bytes.try_into().map_err(|_| ParseError::InvalidData)?);

        self.pointer += 8;
        Ok(val)
    }

    pub fn read_boolean(&mut self) -> Result<bool, ParseError> {
        Ok(self.read_byte()? != 0)
    }

    pub fn read_string(&mut self) -> Result<Option<String>, ParseError> {
        self.check(1)?;
        let prior = self.pointer;

        let flag = self.data[self.pointer];
        self.pointer += 1;

        if flag == 0x00 {
            return Ok(None);
        }

        if flag != 0x0b {
            self.pointer = prior;
            return Err(ParseError::InvalidData);
        }

        let uleb = self.read_uleb();
        if uleb.is_err() {
            self.pointer = prior;
        }

        let len = u64::from(uleb?).try_into().map_err(|_| {
            self.pointer = prior;
            ParseError::InvalidData
        })?;

        if let Err(e) = self.check(len) {
            self.pointer = prior;
            return Err(e);
        }

        let bytes = &self.data[self.pointer..self.pointer + len];
        self.pointer += len;

        Ok(Some(String::from_utf8(bytes.into()).map_err(|_| {
            self.pointer = prior;
            ParseError::InvalidData
        })?))
    }
}
