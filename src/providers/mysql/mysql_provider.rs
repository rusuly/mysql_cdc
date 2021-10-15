use crate::commands::dump_binlog_command::DumpBinlogCommand;
use crate::commands::dump_binlog_gtid_command::DumpBinlogGtidCommand;
use crate::packet_channel::PacketChannel;
use crate::replica_options::ReplicaOptions;
use crate::starting_strategy::StartingStrategy;

pub fn replicate_mysql(channel: &mut PacketChannel, options: &ReplicaOptions, server_id: u32) {
    if options.binlog.starting_strategy == StartingStrategy::FromGtid {
        if let Some(gtid_set) = &options.binlog.gtid_set {
            let command = DumpBinlogGtidCommand::new(
                server_id,
                options.binlog.filename.clone(),
                options.binlog.position,
            );
            channel.write_packet(&command.serialize(&gtid_set), 0)
        } else {
            panic!("GtidSet was not specified");
        }
    } else {
        let command = DumpBinlogCommand::new(
            server_id,
            options.binlog.filename.clone(),
            options.binlog.position,
        );
        channel.write_packet(&command.serialize(), 0)
    }
}
