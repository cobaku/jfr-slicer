use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

const F: u8 = 0x46;
const L: u8 = 0x4c;
const R: u8 = 0x52;
const HEADER_LENGTH: usize = 544;

struct Chunk {
    header: Header,
}

#[derive(Debug)]
struct Header {
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

impl Header {
    fn new(source: &Vec<u8>) -> Self {
        Header {
            major_version: u8_to_u16(source, 4),
            minor_version: u8_to_u16(source, 6),
            size: u8_to_u64(source, 8),
            constant_pool_offset: u8_to_u64(source, 16),
            metadata_offset: u8_to_u64(source, 24),
            start_nanos: u8_to_u64(source, 32),
            duration_nanos: u8_to_u64(source, 40),
            start_ticks: u8_to_u64(source, 48),
            ticks_per_second: u8_to_u64(source, 56),
            features: u8_to_u32(source, 64),
        }
    }

    fn read(source: &mut File, start: &u64) -> Option<Self> {
        source.seek(SeekFrom::Start(*start));
        let mut chunk = source.take(HEADER_LENGTH as u64);
        let mut buffer: Vec<u8> = Vec::new();
        buffer.resize(HEADER_LENGTH, 0);
        let read = chunk.read(&mut buffer).unwrap();
        if read != HEADER_LENGTH {
            return None;
        }
        let header = Header::new(&buffer);
        Some(header)
    }
}

#[derive(Debug)]
struct Metadata {
    size: u32,
    event_type: u64,
    start: u64,
    duration: u64,
    metadata_id: u64,
    string_count: u32,
}

impl Metadata {
    fn new(source: &Vec<u8>) -> Self {
        Metadata {
            size: u8_to_u32(source, 0),
            event_type: 0,
            start: 0,
            duration: 0,
            metadata_id: 0,
            string_count: 0,
        }
    }

    fn read(source: &mut File, start: &u64, size: &u64) -> Option<Self> {
        println!("Size is:{}", size);
        let usize = (*size) as usize;
        source.seek(SeekFrom::Start(*start));
        let mut event = source.take(*size);
        let mut buffer: Vec<u8> = Vec::new();
        buffer.resize(usize, 0);
        let read = event.read(&mut buffer).unwrap();
        if read != usize {
            return None;
        }
        let metadata = Metadata::new(&buffer);
        Some(metadata)
    }
}

struct Event {}

impl Event {
    fn new(source: &Vec<u8>) -> Self {
        Event {}
    }

    fn read(source: &mut File, start: &u64, size: &u64) -> Option<Self> {
        let usize = (*size) as usize;
        source.seek(SeekFrom::Start(*start));
        let mut event = source.take(*size);
        let mut buffer: Vec<u8> = Vec::new();
        buffer.resize(usize, 0);
        let read = event.read(&mut buffer).unwrap();
        if read != usize {
            return None;
        }
        let event = Event::new(&buffer);
        Some(event)
    }
}

fn u8_to_u16(source: &Vec<u8>, index: usize) -> u16 {
    u16::from_be_bytes([source[index], source[index + 1]])
}

fn u8_to_u32(source: &Vec<u8>, index: usize) -> u32 {
    let mut buf = [0u8; 4];
    buf.copy_from_slice(&source[index..index + 4]);
    u32::from_be_bytes(buf)
}

fn u8_to_u64(source: &Vec<u8>, index: usize) -> u64 {
    let mut buf = [0u8; 8];
    buf.copy_from_slice(&source[index..index + 8]);
    u64::from_be_bytes(buf)
}

fn main() {
    let mut input = File::open("file.jfr").unwrap();
    let mut cursor: u64 = 0;
    let mut drained = false;
    let header = Header::read(&mut input, &cursor);
    if header.is_none() {
        drained = true;
    }
    println!("Start: {}, header: {:?}", cursor, header);

    cursor = header.as_ref().unwrap().metadata_offset;
    let metadata_len = header.as_ref().unwrap().size - cursor;
    //cursor = cursor + HEADER_LENGTH as u64;
    //let events = Event::read(&mut input, &cursor);
    let metadata = Metadata::read(&mut input, &cursor, &metadata_len);
    if metadata.is_none() {
        drained = true;
    }
    println!("Metadata: {:?}", metadata);
}
