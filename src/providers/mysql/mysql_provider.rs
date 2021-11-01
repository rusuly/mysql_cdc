use crate::commands::dump_binlog_command::DumpBinlogCommand;
use crate::commands::dump_binlog_gtid_command::DumpBinlogGtidCommand;
use crate::errors::Error;
use crate::packet_channel::PacketChannel;
use crate::replica_options::ReplicaOptions;
use crate::starting_strategy::StartingStrategy;

pub fn replicate_mysql(
    channel: &mut PacketChannel,
    options: &ReplicaOptions,
    server_id: u32,
) -> Result<(), Error> {
    if options.binlog.starting_strategy == StartingStrategy::FromGtid {
        if let Some(gtid_set) = &options.binlog.gtid_set {
            let command = DumpBinlogGtidCommand::new(
                server_id,
                options.binlog.filename.clone(),
                options.binlog.position,
            );
            channel.write_packet(&command.serialize(&gtid_set)?, 0)?
        } else {
            return Err(Error::String("GtidSet was not specified".to_string()));
        }
    } else {
        let command = DumpBinlogCommand::new(
            server_id,
            options.binlog.filename.clone(),
            options.binlog.position,
        );
        channel.write_packet(&command.serialize()?, 0)?
    }
    Ok(())
}
