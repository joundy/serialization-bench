# Compression Test

This project compares different serialization formats and compression algorithms for data storage efficiency. It demonstrates the combination of various serialization methods with different compression algorithms to achieve optimal data size reduction.

## Serialization Methods

The project implements three serialization formats:

1. **JSON (serde_json)**
   - Human-readable text format
   - Used as the baseline for size comparisons
   - Most verbose but most widely supported

2. **Borsh (Binary Object Representation Serializer for Hashing)**
   - Binary format optimized for size and speed
   - Deterministic serialization
   - Typically more compact than JSON

3. **CBOR (Concise Binary Object Representation)**
   - Binary format that balances between readability and size
   - Self-describing format
   - More compact than JSON while maintaining good compatibility

## Compression Algorithms

The following compression algorithms are implemented and can be combined with any of the serialization methods:

1. **Brotli**
   - General-purpose compression
   - Configurable quality (1-11) and window size (10-24)
   - Typically achieves high compression ratios

2. **Gzip**
   - Widely supported compression format
   - Compression levels 0-9
   - Good balance between speed and compression ratio

3. **Deflate**
   - Raw DEFLATE compression without headers
   - Compression levels 0-9
   - Base algorithm used by Gzip and Zlib

4. **Zlib**
   - Similar to Gzip but with different headers
   - Compression levels 0-9
   - Widely used in many applications

5. **LZ4**
   - Fast compression algorithm
   - Compression levels 0-16
   - Optimized for speed over compression ratio

6. **Snap**
   - Simple and fast compression
   - Based on Google's Snappy compression
   - Fixed compression level
   - Focuses on speed rather than maximum compression

## Usage

Run the benchmark to see compression ratios compared to JSON:

```bash
cargo run
```

The output shows a table comparing the size of the data using different combinations of serialization formats and compression algorithms, with the reduction percentage calculated relative to the JSON baseline.

## Results

The results are displayed in a table format showing:
- Method (serialization + compression combination)
- Size in bytes
- Reduction percentage compared to JSON

Methods are sorted by their reduction percentage in descending order, making it easy to identify the most effective combinations for your use case.
