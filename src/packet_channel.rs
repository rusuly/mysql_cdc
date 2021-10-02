use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Read;
use std::io::{Cursor, Write};
use std::net::TcpStream;

use crate::constants::PACKET_HEADER_SIZE;
use crate::replica_options::ReplicaOptions;

pub struct PacketChannel {
    stream: TcpStream,
}

impl PacketChannel {
    pub fn new(options: &ReplicaOptions) -> Self {
        let address: String = format!("{}:{}", options.hostname, options.port.to_string());
        let stream = TcpStream::connect(address).expect("Could not connect to the server");

        Self { stream }
    }

    pub fn read_packet(&mut self) -> (Vec<u8>, u8) {
        let mut header_buffer = [0; PACKET_HEADER_SIZE];

        let packet_size = match self.stream.read_exact(&mut header_buffer) {
            Ok(_x) => {
                let size_buffer = &header_buffer[0..3];
                let mut cursor = Cursor::new(size_buffer);
                cursor.read_u24::<LittleEndian>().unwrap()
            }
            Err(e) => match e.kind() {
                //todo: ErrorKind::UnexpectedEof => return None,
                _ => panic!("Invalid file format"),
            },
        };

        let seq_num = header_buffer[3];

        let mut packet: Vec<u8> = vec![0; packet_size as usize];
        self.stream.read_exact(&mut packet).unwrap();

        (packet, seq_num)
    }

    pub fn write_packet(&mut self, packet: &[u8], seq_num: u8) {
        let packet_len = packet.len() as u32;
        self.stream.write_u24::<LittleEndian>(packet_len).unwrap();
        self.stream.write_u8(seq_num).unwrap();
        self.stream.write(packet).unwrap();
    }

    pub fn upgrade_to_ssl(&mut self) {
        unimplemented!();
    }
}
