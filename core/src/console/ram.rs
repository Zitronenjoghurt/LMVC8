pub const RAM_SIZE: usize = 0x4000; // 16KiB

#[derive(Debug, Clone)]
pub struct RAM {
    data: [u8; RAM_SIZE],
}

impl Default for RAM {
    fn default() -> Self {
        Self {
            data: [0; RAM_SIZE],
        }
    }
}
