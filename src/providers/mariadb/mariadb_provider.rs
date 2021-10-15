use crate::commands::dump_binlog_command::DumpBinlogCommand;
use crate::commands::query_command::QueryCommand;
use crate::commands::register_slave_command::RegisterSlaveCommand;
use crate::extensions::panic_if_error;
use crate::packet_channel::PacketChannel;
use crate::replica_options::ReplicaOptions;
use crate::starting_strategy::StartingStrategy;

pub fn replicate_mariadb(channel: &mut PacketChannel, options: &ReplicaOptions, server_id: u32) {
    let command = QueryCommand::new("SET @mariadb_slave_capability=4".to_string());
    channel.write_packet(&command.serialize(), 0);
    let (packet, _) = channel.read_packet();
    panic_if_error(&packet, "Setting @mariadb_slave_capability error.");

    if options.binlog.starting_strategy == StartingStrategy::FromGtid {
        if let Some(gtid_list) = &options.binlog.gtid_list {
            register_gtid_slave(channel, options.server_id, &gtid_list.to_string());
        } else {
            panic!("GtidList was not specified");
        }
    }

    let command = DumpBinlogCommand::new(
        server_id,
        options.binlog.filename.clone(),
        options.binlog.position,
    );
    channel.write_packet(&command.serialize(), 0)
}

fn register_gtid_slave(channel: &mut PacketChannel, server_id: u32, gtid_list: &String) {
    let command = QueryCommand::new(format!("SET @slave_connect_state='{}'", gtid_list));
    channel.write_packet(&command.serialize(), 0);
    let (packet, _) = channel.read_packet();
    panic_if_error(&packet, "Setting @slave_connect_state error.");

    let command = QueryCommand::new("SET @slave_gtid_strict_mode=0".to_string());
    channel.write_packet(&command.serialize(), 0);
    let (packet, _) = channel.read_packet();
    panic_if_error(&packet, "Setting @slave_gtid_strict_mode error.");

    let command = QueryCommand::new("SET @slave_gtid_ignore_duplicates=0".to_string());
    channel.write_packet(&command.serialize(), 0);
    let (packet, _) = channel.read_packet();
    panic_if_error(&packet, "Setting @slave_gtid_ignore_duplicates error.");

    let command = RegisterSlaveCommand::new(server_id);
    channel.write_packet(&command.serialize(), 0);
    let (packet, _) = channel.read_packet();
    panic_if_error(&packet, "Registering slave error.");
}
