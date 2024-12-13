pub trait Compression: Send + Sync {
    fn compress(&mut self, data: &[u8]) -> std::io::Result<Vec<u8>>;
    fn decompress(&mut self, data: &[u8]) -> std::io::Result<Vec<u8>>;
    fn clone_box(&self) -> Box<dyn Compression>;
}

impl Clone for Box<dyn Compression> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
