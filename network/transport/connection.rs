use crate::network::transport::stream::Stream;

pub struct Connection {
    pub id: String,
    pub stream: Stream,
}

impl Connection {
    pub fn new(id: &str, stream: Stream) -> Self {
        Self {
            id: id.to_string(),
            stream,
        }
    }

    pub fn send(&mut self, data: &[u8]) {
        self.stream.write(data);
    }

    pub fn receive(&mut self) -> Vec<u8> {
        self.stream.read()
    }
}
