pub struct ByteReader<'a> {
    buffer: &'a [u8],
    position: usize,
}

impl<'a> ByteReader<'a> {
    pub fn of(buffer: &'a [u8]) -> Self {
        ByteReader {
            buffer,
            position: 0,
        }
    }

    pub fn read_u8(&mut self) -> u8 {
        let value = self.buffer[self.position];
        self.position += 1;
        value
    }

    pub fn read_u16(&mut self) -> u16 {
        let value =
            u16::from_le_bytes([self.buffer[self.position], self.buffer[self.position + 1]]);
        self.position += 2;
        value
    }

    pub fn read_u32(&mut self) -> u32 {
        let value = u32::from_le_bytes([
            self.buffer[self.position],
            self.buffer[self.position + 1],
            self.buffer[self.position + 2],
            self.buffer[self.position + 3],
        ]);
        self.position += 4;
        value
    }

    pub fn read_enum_u8<T>(&mut self) -> T
    where
        T: From<u8>,
    {
        T::from(self.read_u8())
    }

    pub fn read_enum_u16<T>(&mut self) -> T
    where
        T: From<u16>,
    {
        T::from(self.read_u16())
    }

    pub fn read_enum_u32<T>(&mut self) -> T
    where
        T: From<u32>,
    {
        T::from(self.read_u32())
    }

    pub fn read_slice<const L: usize>(&mut self) -> [u8; L] {
        let value = &self.buffer[self.position..self.position + L];
        self.position += L;
        value.try_into().unwrap()
    }
}
