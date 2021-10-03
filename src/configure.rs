use crate::binlog_client::BinlogClient;
use crate::commands::query_command::QueryCommand;
use crate::constants::checksum_type::ChecksumType;
use crate::extensions::panic_if_error;
use crate::packet_channel::PacketChannel;
use crate::responses::response_type::ResponseType;
use crate::responses::result_set_row_packet::ResultSetRowPacket;
use crate::starting_strategy::StartingStrategy;

impl BinlogClient {
    pub fn adjust_starting_position(&mut self, channel: &mut PacketChannel) {
        if self.options.binlog.starting_strategy != StartingStrategy::FromEnd {
            return;
        }

        // Ignore if position was read before in case of reconnect.
        if !self.options.binlog.filename.is_empty() {
            return;
        }

        let command = QueryCommand::new("show master status".to_string());
        channel.write_packet(&command.serialize(), 0);

        let result_set = self.read_result_set(channel);
        if result_set.len() != 1 {
            panic!("Could not read master binlog position.");
        }

        self.options.binlog.filename = result_set[0].cells[0].clone();
        self.options.binlog.position = result_set[0].cells[1].parse().unwrap();
    }

    pub fn set_master_heartbeat(&mut self, channel: &mut PacketChannel) {
        let milliseconds = self.options.heartbeat_interval.as_millis();
        let nanoseconds = milliseconds * 1000 * 1000;
        let query = format!("set @master_heartbeat_period={}", nanoseconds);
        let command = QueryCommand::new(query.to_string());
        channel.write_packet(&command.serialize(), 0);
        let (packet, _) = channel.read_packet();
        panic_if_error(&packet, "Setting master heartbeat error.");
    }

    pub fn set_master_binlog_checksum(&mut self, channel: &mut PacketChannel) {
        let command =
            QueryCommand::new("SET @master_binlog_checksum= @@global.binlog_checksum".to_string());
        channel.write_packet(&command.serialize(), 0);
        let (packet, _) = channel.read_packet();
        panic_if_error(&packet, "Setting master_binlog_checksum error.");

        let command = QueryCommand::new("SELECT @master_binlog_checksum".to_string());
        channel.write_packet(&command.serialize(), 0);
        let result_set = self.read_result_set(channel);

        // When replication is started fake RotateEvent comes before FormatDescriptionEvent.
        // In order to deserialize the event we have to obtain checksum type length in advance.
        self.parser.checksum_type = ChecksumType::from_name(&result_set[0].cells[0]);
    }

    fn read_result_set(&self, channel: &mut PacketChannel) -> Vec<ResultSetRowPacket> {
        let (packet, _) = channel.read_packet();
        panic_if_error(&packet, "Reading result set error.");

        loop {
            // Skip through metadata
            let (packet, _) = channel.read_packet();
            if packet[0] == ResponseType::END_OF_FILE {
                break;
            }
        }

        let mut result_set = vec![];
        loop {
            let (packet, _) = channel.read_packet();
            panic_if_error(&packet, "Query result set error.");
            if packet[0] == ResponseType::END_OF_FILE {
                break;
            }
            result_set.push(ResultSetRowPacket::parse(&packet));
        }
        result_set
    }
}
