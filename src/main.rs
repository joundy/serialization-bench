mod broli;
mod deflate;
mod gzip;
mod lz4;
mod snap;
mod types;
mod zlib;

use prettytable::{row, Table};
use types::Compression;

use borsh::{from_slice, io, to_vec, BorshDeserialize, BorshSerialize};
use broli::Brotli;
use deflate::Deflate;
use gzip::Gzip;
use lz4::Lz4;
use snap::Snap;
use zlib::Zlib;
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
    // Structure to hold row data
    #[derive(Clone)]
    struct TableRow {
        method: String,
        bytes: usize,
        reduction: i32,
    }

    let mut rows: Vec<TableRow> = Vec::new();
    let a = Sample {
        long_field_a: "String A".to_string(),
        long_field_b: "String B".to_string(),
        long_field_c: "String C".to_string(),
        long_field_x: 1,
        long_field_y: 1,
        long_field_z: u64::MAX,
    };

    let compression_benchs: Vec<CompressionBench> = vec![
        CompressionBench {
            name: "brotli".to_string(),
            compression: Box::new(Brotli::new(11, 22, 4096)),
        },
        CompressionBench {
            name: "gzip".to_string(),
            compression: Box::new(Gzip::new(9)),
        },
        CompressionBench {
            name: "deflate".to_string(),
            compression: Box::new(Deflate::new(9)),
        },
        CompressionBench {
            name: "zlib".to_string(),
            compression: Box::new(Zlib::new(9)),
        },
        CompressionBench {
            name: "lz4".to_string(),
            compression: Box::new(Lz4::new(16)),
        },
        CompressionBench {
            name: "snap".to_string(),
            compression: Box::new(Snap::new()),
        },
    ];

    let encoded_json_len = serde_json::to_vec(&a).unwrap().len();
    rows.push(TableRow {
        method: "json".to_string(),
        bytes: encoded_json_len,
        reduction: 0,
    });

    let json_reduction = |value: usize| -> i32 {
        let percent =
            ((encoded_json_len as f32) - (value as f32)) / encoded_json_len as f32 * 100.0;
        percent as i32
    };

    // std borsh
    {
        let encoded_borsh = to_vec(&a).unwrap();
        rows.push(TableRow {
            method: "borsh".to_string(),
            bytes: encoded_borsh.len(),
            reduction: json_reduction(encoded_borsh.len()),
        });

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
        rows.push(TableRow {
            method: format!("borsh({})", bench.name),
            bytes: compressed.len(),
            reduction: json_reduction(compressed.len()),
        });
    }

    // std cbor
    {
        let encoded_cbor = serde_cbor::to_vec(&a).unwrap();
        rows.push(TableRow {
            method: "cbor".to_string(),
            bytes: encoded_cbor.len(),
            reduction: json_reduction(encoded_cbor.len()),
        });
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
        rows.push(TableRow {
            method: format!("cbor({})", bench.name),
            bytes: compressed.len(),
            reduction: json_reduction(compressed.len()),
        });
    }

    // Sort rows by reduction value in descending order
    rows.sort_by(|a, b| b.reduction.cmp(&a.reduction));

    // Create and populate the table with sorted rows
    let mut table = Table::new();
    table.add_row(row!["Method", "Bytes", "Reduction from json (%)"]);
    
    for row in rows {
        table.add_row(row![
            row.method,
            row.bytes,
            row.reduction
        ]);
    }

    // Print the table to stdout
    table.printstd();
}
