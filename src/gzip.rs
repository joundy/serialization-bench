use crate::types::Compression;
use flate2::{read::GzDecoder, write::GzEncoder, Compression as GzCompression};
use std::io::{Read, Write};

#[derive(Copy, Clone)]
pub struct Gzip {
    level: u32, // 0-9, where 0 is no compression and 9 is max compression
}

impl Gzip {
    pub fn new(level: u32) -> Self {
        assert!(level <= 9);
        Gzip { level }
    }
}

impl Compression for Gzip {
    fn compress(&mut self, data: &[u8]) -> std::io::Result<Vec<u8>> {
        let mut encoder = GzEncoder::new(Vec::new(), GzCompression::new(self.level));
        encoder.write_all(data)?;
        encoder.finish()
    }

    fn decompress(&mut self, data: &[u8]) -> std::io::Result<Vec<u8>> {
        let mut decoder = GzDecoder::new(data);
        let mut decompressed_data = Vec::new();
        decoder.read_to_end(&mut decompressed_data)?;
        Ok(decompressed_data)
    }

    fn clone_box(&self) -> Box<dyn Compression> {
        Box::new(self.clone())
    }
}
