#[derive(Debug)]
pub enum DatabaseProvider {
    MariaDB,
    MySQL,
}

impl DatabaseProvider {
    pub fn from(server: &String) -> Self {
        match server.contains("MariaDB") {
            true => DatabaseProvider::MariaDB,
            _ => DatabaseProvider::MySQL,
        }
    }
}
