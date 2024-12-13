use crate::types::Compression;
use flate2::{read::DeflateDecoder, write::DeflateEncoder, Compression as DeflateCompression};
use std::io::{Read, Write};

#[derive(Copy, Clone)]
pub struct Deflate {
    level: u32, // 0-9, where 0 is no compression and 9 is max compression
}

impl Deflate {
    pub fn new(level: u32) -> Self {
        assert!(level <= 9);
        Deflate { level }
    }
}

impl Compression for Deflate {
    fn compress(&mut self, data: &[u8]) -> std::io::Result<Vec<u8>> {
        let mut encoder = DeflateEncoder::new(Vec::new(), DeflateCompression::new(self.level));
        encoder.write_all(data)?;
        encoder.finish()
    }

    fn decompress(&mut self, data: &[u8]) -> std::io::Result<Vec<u8>> {
        let mut decoder = DeflateDecoder::new(data);
        let mut decompressed_data = Vec::new();
        decoder.read_to_end(&mut decompressed_data)?;
        Ok(decompressed_data)
    }

    fn clone_box(&self) -> Box<dyn Compression> {
        Box::new(self.clone())
    }
}
