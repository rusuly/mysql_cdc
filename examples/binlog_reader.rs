use mysql_cdc::{binlog_reader::BinlogReader, errors::Error};
use std::fs::File;

const PATH: &str = "mysql-bin.000001";

fn main() -> Result<(), Error> {
    let file = File::open(PATH)?;
    let reader = BinlogReader::new(file)?;

    for result in reader.read_events() {
        let (header, event) = result?;
        println!("{:#?}", header);
        println!("{:#?}", event);
    }
    Ok(())
}
