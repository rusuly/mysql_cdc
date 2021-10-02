use mysql_cdc::binlog_client::BinlogClient;
use mysql_cdc::replica_options::ReplicaOptions;
use mysql_cdc::ssl_mode::SslMode;

fn main() {
    let options = ReplicaOptions {
        username: String::from("root"),
        password: String::from("Qwertyu1"),
        blocking: true,
        ssl_mode: SslMode::DISABLED,
        ..Default::default()
    };

    let client = BinlogClient::new(options);

    for (header, binlog_event) in client.replicate() {
        println!("{:#?}", header);
        println!("{:#?}", binlog_event);
    }
}
