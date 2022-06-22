use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

const F: u8 = 0x46;
const L: u8 = 0x4c;
const R: u8 = 0x52;
const HEADER_LENGTH: usize = 544;

struct Chunk {
    header: ChunkHeader,
}

#[derive(Debug)]
struct ChunkHeader {
    major_version: u16,
    minor_version: u16,
    size: u64,
    constant_pool_offset: u64,
    metadata_offset: u64,
    start_nanos: u64,
    duration_nanos: u64,
    start_ticks: u64,
    ticks_per_second: u64,
    features: u32,
}

fn u8_to_u16(source: &Vec<u8>, index: usize) -> u16 {
    u16::from_be_bytes([source[index], source[index + 1]])
}

fn u8_to_u64(source: &Vec<u8>, index: usize) -> u64 {
    u64::from_be_bytes([
        source[index],
        source[index + 1],
        source[index + 2],
        source[index + 3],
        source[index + 4],
        source[index + 5],
        source[index + 6],
        source[index + 7]
    ])
}

fn parse_header(source: &Vec<u8>) -> ChunkHeader {
    ChunkHeader {
        major_version: u8_to_u16(source, 4),
        minor_version: u8_to_u16(source, 6),
        size: u8_to_u64(source, 16),
        constant_pool_offset: u8_to_u64(source, 24),
        metadata_offset: u8_to_u64(source, 32),
        start_nanos: u8_to_u64(source, 40),
        duration_nanos: u8_to_u64(source, 48),
        start_ticks: u8_to_u64(source, 56),
        ticks_per_second: u8_to_u64(source, 64),
        features: 0,
    }
}

fn read_chunk(source: &mut File, start: u64) -> (u64, ChunkHeader) {
    source.seek(SeekFrom::Start(start));
    let mut chunk = source.take(HEADER_LENGTH as u64);
    let mut header: Vec<u8> = Vec::new();
    header.resize(HEADER_LENGTH, 0);
    let read = chunk.read(&mut header).unwrap();
    if read != HEADER_LENGTH {
        panic!("Buffer underflow!");
    }
    (0, parse_header(&header))
}

fn main() {
    let mut input = File::open("file.jfr").unwrap();
    let mut start: u64 = 0;
    let mut drained = false;
    while !drained {
        let (new, _header) = read_chunk(&mut input, start);
        println!("{:?}", _header);
        if new >= 0 {
            drained = true;
            return;
        }
        start = new
    }
}
