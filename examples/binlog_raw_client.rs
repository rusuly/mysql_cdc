use mysql_cdc::binlog_client::BinlogClient;
use mysql_cdc::binlog_options::BinlogOptions;
use mysql_cdc::errors::Error;
use mysql_cdc::providers::mariadb::gtid::gtid_list::GtidList;
use mysql_cdc::providers::mysql::gtid::gtid_set::GtidSet;
use mysql_cdc::replica_options::ReplicaOptions;
use mysql_cdc::ssl_mode::SslMode;

fn main() -> Result<(), Error> {
    // Start replication from first event of first available master binlog.
    // Note that binlog files by default have expiration time and deleted.
    let options = BinlogOptions::from_start();

    let options = ReplicaOptions {
        username: String::from("root"),
        password: String::from("password"),
        blocking: true,
        ssl_mode: SslMode::Disabled,
        binlog: options,
        ..Default::default()
    };

    let mut client = BinlogClient::new(options);

    for result in client.replicate_raw()? {
        let (header, data) = result?;
        println!("Header: {:#?}", header);
        println!("Event: {:#?}", data);

        println!("Replication position before event processed");
        print_position(&client);
    }
    Ok(())
}

fn print_position(client: &BinlogClient) {
    println!("Binlog Filename: {:#?}", client.options.binlog.filename);
    println!("Binlog Position: {:#?}", client.options.binlog.position);

    if let Some(x) = &client.options.binlog.gtid_list {
        println!("MariaDB Gtid Position: {:#?}", x.to_string());
    }
    if let Some(x) = &client.options.binlog.gtid_set {
        println!("MySQL Gtid Position: {:#?}", x.to_string());
    }
}
