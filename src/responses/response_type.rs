pub mod ResponseType {
    pub const OK: u8 = 0x00;
    pub const ERROR: u8 = 0xFF;
    pub const END_OF_FILE: u8 = 0xFE;
    pub const AUTH_PLUGIN_SWITCH: u8 = 0xFE;
}
