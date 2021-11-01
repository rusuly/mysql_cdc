use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Read, Write};
use std::net::TcpStream;

use crate::constants::PACKET_HEADER_SIZE;
use crate::replica_options::ReplicaOptions;

pub struct PacketChannel {
    stream: TcpStream,
}

impl PacketChannel {
    pub fn new(options: &ReplicaOptions) -> Result<Self, io::Error> {
        let address: String = format!("{}:{}", options.hostname, options.port.to_string());
        let stream = TcpStream::connect(address)?;
        Ok(Self { stream })
    }

    pub fn read_packet(&mut self) -> Result<(Vec<u8>, u8), io::Error> {
        let mut header_buffer = [0; PACKET_HEADER_SIZE];

        self.stream.read_exact(&mut header_buffer)?;
        let packet_size = (&header_buffer[0..3]).read_u24::<LittleEndian>()?;
        let seq_num = header_buffer[3];

        let mut packet: Vec<u8> = vec![0; packet_size as usize];
        self.stream.read_exact(&mut packet)?;

        Ok((packet, seq_num))
    }

    pub fn write_packet(&mut self, packet: &[u8], seq_num: u8) -> Result<(), io::Error> {
        let packet_len = packet.len() as u32;
        self.stream.write_u24::<LittleEndian>(packet_len)?;
        self.stream.write_u8(seq_num)?;
        self.stream.write(packet)?;
        Ok(())
    }

    pub fn upgrade_to_ssl(&mut self) {
        unimplemented!();
    }
}
