use crate::types::Compression;
use snap::raw::{Decoder, Encoder};

#[derive(Copy, Clone)]
pub struct Snap;

impl Snap {
    pub fn new() -> Self {
        Snap
    }
}

impl Compression for Snap {
    fn compress(&mut self, data: &[u8]) -> std::io::Result<Vec<u8>> {
        let mut encoder = Encoder::new();
        Ok(encoder.compress_vec(data)?)
    }

    fn decompress(&mut self, data: &[u8]) -> std::io::Result<Vec<u8>> {
        let mut decoder = Decoder::new();
        Ok(decoder.decompress_vec(data)?)
    }

    fn clone_box(&self) -> Box<dyn Compression> {
        Box::new(self.clone())
    }
}
