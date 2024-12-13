use crate::types::Compression;
use lz4::{Decoder, EncoderBuilder};
use std::io::{Read, Write};

#[derive(Copy, Clone)]
pub struct Lz4 {
    level: u32, // 0-16, where 0 is no compression and 16 is max compression
}

impl Lz4 {
    pub fn new(level: u32) -> Self {
        assert!(level <= 16);
        Lz4 { level }
    }
}

impl Compression for Lz4 {
    fn compress(&mut self, data: &[u8]) -> std::io::Result<Vec<u8>> {
        let mut encoder = EncoderBuilder::new()
            .level(self.level)
            .build(Vec::new())?;
        encoder.write_all(data)?;
        let (compressed_data, _result) = encoder.finish();
        Ok(compressed_data)
    }

    fn decompress(&mut self, data: &[u8]) -> std::io::Result<Vec<u8>> {
        let mut decoder = Decoder::new(data)?;
        let mut decompressed_data = Vec::new();
        decoder.read_to_end(&mut decompressed_data)?;
        Ok(decompressed_data)
    }

    fn clone_box(&self) -> Box<dyn Compression> {
        Box::new(self.clone())
    }
}
