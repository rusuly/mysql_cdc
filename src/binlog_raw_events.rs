//
// binlog_raw_events.rs
// Copyright (C) 2022 db3.network Author imotai <codego.me@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
use crate::errors::Error;
use crate::events::event_header::EventHeader;
use crate::packet_channel::PacketChannel;
use crate::responses::end_of_file_packet::EndOfFilePacket;
use crate::responses::error_packet::ErrorPacket;
use crate::responses::response_type::ResponseType;

pub struct BinlogRawEvents {
    pub channel: PacketChannel,
}

impl BinlogRawEvents {
    pub fn new(channel: PacketChannel) -> Self {
        Self { channel }
    }
}

impl Iterator for BinlogRawEvents {
    type Item = Result<(EventHeader, Vec<u8>), Error>;

    /// Reads binlog event packets from network stream.
    /// <a href="https://mariadb.com/kb/en/3-binlog-network-stream/">See more</a>
    fn next(&mut self) -> Option<Self::Item> {
        let (packet, _) = match self.channel.read_packet() {
            Ok(x) => x,
            Err(e) => return Some(Err(Error::IoError(e))),
        };
        match packet[0] {
            ResponseType::OK => match EventHeader::parse(&packet[1..]) {
                Ok(h) => Some(Ok((h, packet))),
                Err(e) => Some(Err(e)),
            },
            ResponseType::ERROR => match ErrorPacket::parse(&packet[1..]) {
                Ok(er) => Some(Err(Error::String(format!("Event stream error. {:?}", er)))),
                Err(e) => Some(Err(Error::IoError(e))),
            },
            ResponseType::END_OF_FILE => {
                let _ = EndOfFilePacket::parse(&packet[1..]);
                None
            }
            _ => Some(Err(Error::String(
                "Unknown network stream status".to_string(),
            ))),
        }
    }
}
