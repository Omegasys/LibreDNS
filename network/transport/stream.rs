pub struct Stream {
    buffer: Vec<u8>,
}

impl Stream {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
        }
    }

    pub fn write(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }

    pub fn read(&mut self) -> Vec<u8> {
        let data = self.buffer.clone();
        self.buffer.clear();
        data
    }
}
