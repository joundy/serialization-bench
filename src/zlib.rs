use crate::types::Compression;
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression as ZlibCompression};
use std::io::{Read, Write};

#[derive(Copy, Clone)]
pub struct Zlib {
    level: u32, // 0-9, where 0 is no compression and 9 is max compression
}

impl Zlib {
    pub fn new(level: u32) -> Self {
        assert!(level <= 9);
        Zlib { level }
    }
}

impl Compression for Zlib {
    fn compress(&mut self, data: &[u8]) -> std::io::Result<Vec<u8>> {
        let mut encoder = ZlibEncoder::new(Vec::new(), ZlibCompression::new(self.level));
        encoder.write_all(data)?;
        encoder.finish()
    }

    fn decompress(&mut self, data: &[u8]) -> std::io::Result<Vec<u8>> {
        let mut decoder = ZlibDecoder::new(data);
        let mut decompressed_data = Vec::new();
        decoder.read_to_end(&mut decompressed_data)?;
        Ok(decompressed_data)
    }

    fn clone_box(&self) -> Box<dyn Compression> {
        Box::new(self.clone())
    }
}
