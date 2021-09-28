use mysql_cdc::options::ReplicaOptions;
use mysql_cdc::replicate;
use mysql_cdc::ssl_mode::SslMode;

fn main() {
    let options = ReplicaOptions {
        username: String::from("root"),
        password: String::from("Qwertyu1"),
        blocking: true,
        ..Default::default()
    };
    replicate(&options);
}
