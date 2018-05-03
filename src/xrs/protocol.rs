// Connection request
pub const CONNECT_LSB: u8 = 0x6C; // Intel x86, AMD64/x86-64
pub const CONNECT_MSB: u8 = 0x42; // Others
pub const CONNECT_MAJOR: u16 = 11; // Server major version
pub const CONNECT_MINOR: u16 = 0; // Server minor version
pub const CONNECT_AUTH_NAME: &str = ""; // Auth protocol name
pub const CONNECT_AUTH_DATA: &str = ""; // Auth protocol value
pub const CONNECT_FAILED: u8 = 0;
pub const CONNECT_SUCCESS: u8 = 1;
pub const CONNECT_AUTHENTICATE: u8 = 2;

// Opcodes
pub const OP_TODO: u8 = 0;
