pub mod ehdr;
pub mod file;
pub mod phdr;
pub mod rela;
pub mod shdr;
pub mod strtab;
pub mod sym;

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

#[cfg(test)]
mod tests {
    use crate::{make_u16, make_u32, make_u64};

    #[test]
    fn test_endianess() {
        let buf: [u8; 8] = [0x0c, 0x00, 0xf, 0xf, 0xe, 0xe, 0x0, 0x0];
        assert_eq!(make_u16(&buf, 0), 0x0c);
        assert_eq!(make_u32(&buf, 0), 0x0f0f000c);
        assert_eq!(make_u64(&buf, 0), 0x0e0e0f0f000c);
    }
}
