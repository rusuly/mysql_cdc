pub mod ResponseType {
    pub const Ok: u8 = 0x00;
    pub const Error: u8 = 0xFF;
    pub const EndOfFile: u8 = 0xFE;
    pub const AuthPluginSwitch: u8 = 0xFE;
}
