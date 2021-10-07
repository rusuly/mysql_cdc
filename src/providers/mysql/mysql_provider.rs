use crate::commands::dump_binlog_command::DumpBinlogCommand;
use crate::packet_channel::PacketChannel;
use crate::replica_options::ReplicaOptions;
use crate::starting_strategy::StartingStrategy;

pub fn replicate_mysql(channel: &mut PacketChannel, options: &ReplicaOptions, server_id: u32) {
    if options.binlog.starting_strategy == StartingStrategy::FromGtid {
        unimplemented!("DumpBinlogGtidCommand")
    } else {
        let command = DumpBinlogCommand::new(
            server_id,
            options.binlog.filename.clone(),
            options.binlog.position,
        );
        channel.write_packet(&command.serialize(), 0)
    }
}
