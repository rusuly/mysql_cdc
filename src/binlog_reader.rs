use crate::constants;
use crate::errors::Error;
use crate::events::binlog_event::BinlogEvent;
use crate::events::event_header::EventHeader;
use crate::events::event_parser::EventParser;
use constants::EVENT_HEADER_SIZE;
use std::fs::File;
use std::io::{ErrorKind, Read};

const MAGIC_NUMBER: [u8; constants::FIRST_EVENT_POSITION] = [0xfe, 0x62, 0x69, 0x6e];

/// Reads binlog events from a stream.
pub struct BinlogReader {
    stream: File,
    parser: EventParser,
    payload_buffer: Vec<u8>,
}

impl BinlogReader {
    pub fn new(mut stream: File) -> Result<Self, Error> {
        let mut header = [0; constants::FIRST_EVENT_POSITION];
        stream.read_exact(&mut header)?;

        if header != MAGIC_NUMBER {
            return Err(Error::String("Invalid binary log file header".to_string()));
        }

        Ok(Self {
            stream,
            parser: EventParser::new(),
            payload_buffer: vec![0; constants::PAYLOAD_BUFFER_SIZE],
        })
    }

    pub fn read_events(self) -> Self {
        self
    }

    pub fn read_event(&mut self) -> Result<(EventHeader, BinlogEvent), Error> {
        // Parse header
        let mut header_buffer = [0; EVENT_HEADER_SIZE];
        self.stream.read_exact(&mut header_buffer)?;
        let header = EventHeader::parse(&header_buffer)?;

        let payload_length = header.event_length as usize - EVENT_HEADER_SIZE;
        if payload_length as usize > constants::PAYLOAD_BUFFER_SIZE {
            let mut vec: Vec<u8> = vec![0; payload_length];

            self.stream.read_exact(&mut vec)?;
            let binlog_event = self.parser.parse_event(&header, &vec)?;
            Ok((header, binlog_event))
        } else {
            let slice = &mut self.payload_buffer[0..payload_length];

            self.stream.read_exact(slice)?;
            let binlog_event = self.parser.parse_event(&header, slice)?;
            Ok((header, binlog_event))
        }
    }
}

impl Iterator for BinlogReader {
    type Item = Result<(EventHeader, BinlogEvent), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.read_event();
        if let Err(error) = &result {
            if let Error::IoError(io_error) = error {
                if let ErrorKind::UnexpectedEof = io_error.kind() {
                    return None;
                }
            }
        }
        Some(result)
    }
}
