pub trait Compression {
    fn compress(&mut self, data: &[u8]) -> std::io::Result<Vec<u8>>;
    fn decompress(&mut self, data: &[u8]) -> std::io::Result<Vec<u8>>;
}
