#[derive(Debug)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

#[derive(Debug)]
pub struct Time {
    pub hour: i16, // Signed value from -838 to 838
    pub minute: u8,
    pub second: u8,
    pub millis: u32,
}

#[derive(Debug)]
pub struct DateTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub millis: u32,
}

#[derive(Debug)]
pub enum MySqlValue {
    TinyInt(u8),
    SmallInt(u16),
    MediumInt(u32),
    Int(u32),
    BigInt(u64),
    Float(f32),
    Double(f64),
    Decimal(String),
    String(String),
    Bit(Vec<bool>),
    Enum(u32),
    Set(u64),
    Blob(Vec<u8>),
    Year(u16),
    Date(Date),
    Time(Time),
    DateTime(DateTime),
    Timestamp(u64), // millis from unix time
}
