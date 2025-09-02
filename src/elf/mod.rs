pub mod ehdr;
pub mod phdr;
pub mod shdr;

pub fn make_u16(b: &[u8], offset: u32) -> u16 {
    let mut buf: [u8; 2] = [0; 2];

    for i in 0..2 {
        buf[i] = b[(offset as usize) + i];
    }

    u16::from_le_bytes(buf)
}

pub fn make_u32(b: &[u8], offset: u32) -> u32 {
    let mut buf: [u8; 4] = [0; 4];

    for i in 0..4 {
        buf[i] = b[(offset as usize) + i];
    }

    u32::from_le_bytes(buf)
}

pub fn make_u64(b: &[u8], offset: u32) -> u64 {
    let mut buf: [u8; 8] = [0; 8];

    for i in 0..8 {
        buf[i] = b[(offset as usize) + i];
    }

    u64::from_le_bytes(buf)
}
