use mysql_cdc::binlog_client::BinlogClient;
use mysql_cdc::binlog_options::BinlogOptions;
use mysql_cdc::errors::Error;
use mysql_cdc::providers::mariadb::gtid::gtid_list::GtidList;
use mysql_cdc::providers::mysql::gtid::gtid_set::GtidSet;
use mysql_cdc::replica_options::ReplicaOptions;
use mysql_cdc::ssl_mode::SslMode;

fn main() -> Result<(), Error> {
    // Start replication from MariaDB GTID
    let _options = BinlogOptions::from_mariadb_gtid(GtidList::parse("0-1-270")?);

    // Start replication from MySQL GTID
    let gtid_set =
        "d4c17f0c-4f11-11ea-93e3-325d3e1cd1c8:1-107, f442510a-2881-11ea-b1dd-27916133dbb2:1-7";
    let _options = BinlogOptions::from_mysql_gtid(GtidSet::parse(gtid_set)?);

    // Start replication from the position
    let _options = BinlogOptions::from_position(String::from("mysql-bin.000008"), 195);

    // Start replication from last master position.
    // Useful when you are only interested in new changes.
    let _options = BinlogOptions::from_end();

    // Start replication from first event of first available master binlog.
    // Note that binlog files by default have expiration time and deleted.
    let options = BinlogOptions::from_start();

    let options = ReplicaOptions {
        username: String::from("root"),
        password: String::from("Qwertyu1"),
        blocking: true,
        ssl_mode: SslMode::Disabled,
        binlog: options,
        ..Default::default()
    };

    let mut client = BinlogClient::new(options);

    for result in client.replicate()? {
        let (header, event) = result?;
        println!("Header: {:#?}", header);
        println!("Event: {:#?}", event);

        println!("Replication position before event processed");
        print_position(&client);

        // After you processed the event, you need to update replication position
        client.commit(&header, &event);

        println!("Replication position after event processed");
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
