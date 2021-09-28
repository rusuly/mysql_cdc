use mysql_cdc::binlog_reader::BinlogReader;
use std::fs::File;

const PATH: &str = "mysql-bin.000001";

fn main() {
    let file = File::open(PATH).unwrap();
    let reader = BinlogReader::new(file).unwrap();

    for (header, binlog_event) in reader.read_events() {
        println!("{:#?}", header);
        println!("{:#?}", binlog_event);
    }
}
