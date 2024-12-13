mod broli;
mod types;

use prettytable::{row, Table};
use types::Compression;

use borsh::{from_slice, io, to_vec, BorshDeserialize, BorshSerialize};
use broli::Brotli;
use serde::{Deserialize, Serialize};
use serde_cbor;
use serde_json;

// CBOR doens't support u128, so we use u64
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Debug)]
struct Sample {
    long_field_a: String,
    long_field_b: String,
    long_field_c: String,
    long_field_x: u64,
    long_field_y: u64,
    long_field_z: u64,
}

#[derive(Clone)]
struct CompressionBench {
    name: String,
    compression: Box<dyn Compression>,
}

fn main() {
    let mut table = Table::new();
    table.add_row(row!["Method", "Bytes", "Reduction from json (%)"]);

    let a = Sample {
        long_field_a: "String A".to_string(),
        long_field_b: "String B".to_string(),
        long_field_c: "String C".to_string(),
        long_field_x: 1,
        long_field_y: 1,
        long_field_z: u64::MAX,
    };

    let brotli = Brotli::new(11, 22, 4096);

    let compression_benchs: Vec<CompressionBench> = vec![CompressionBench {
        name: "brotli".to_string(),
        compression: Box::new(brotli),
    }];

    let encoded_json_len = serde_json::to_vec(&a).unwrap().len();
    table.add_row(row!["json", encoded_json_len, 0]);

    let json_reduction = |value: usize| -> u32 {
        let percent =
            ((encoded_json_len as f32) - (value as f32)) / encoded_json_len as f32 * 100.0;
        percent as u32
    };

    // std borsh
    {
        let encoded_borsh = to_vec(&a).unwrap();
        table.add_row(row![
            "borsh",
            encoded_borsh.len(),
            json_reduction(encoded_borsh.len())
        ]);

        let decoded_a = from_slice::<Sample>(&encoded_borsh).unwrap();
        assert_eq!(a, decoded_a);
    }

    // compressed
    for mut bench in compression_benchs.clone() {
        let encoded_borsh = to_vec(&a).unwrap();

        let compressed = bench.compression.compress(&encoded_borsh).unwrap();
        let decompressed = bench.compression.decompress(&compressed).unwrap();

        let decoded_a = from_slice::<Sample>(&decompressed).unwrap();
        assert_eq!(a, decoded_a);
        table.add_row(row![
            format!("borsh({})", bench.name),
            compressed.len(),
            json_reduction(compressed.len())
        ]);
    }

    // std cbor
    {
        let encoded_cbor = serde_cbor::to_vec(&a).unwrap();
        table.add_row(row![
            "cbor",
            encoded_cbor.len(),
            json_reduction(encoded_cbor.len())
        ]);
        let decoded_a = serde_cbor::from_slice::<Sample>(&encoded_cbor).unwrap();
        assert_eq!(a, decoded_a);
    }

    // compressed cbor
    for mut bench in compression_benchs {
        let encoded_cbor = serde_cbor::to_vec(&a).unwrap();

        let compressed = bench.compression.compress(&encoded_cbor).unwrap();
        let decompressed = bench.compression.decompress(&compressed).unwrap();

        let decoded_a = serde_cbor::from_slice::<Sample>(&decompressed).unwrap();
        assert_eq!(a, decoded_a);
        table.add_row(row![
            format!("cbor({})", bench.name),
            compressed.len(),
            json_reduction(compressed.len())
        ]);
    }

    // Print the table to stdout
    table.printstd();
}
