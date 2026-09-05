#![allow(dead_code)]
pub const PAGE_SIZE: usize = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PageId(pub u64);

impl PageId {
    pub fn as_offset(&self) -> u64 {
        self.0 * PAGE_SIZE as u64
    }
}

#[derive(Clone)]
pub(crate) struct Page {
    data: [u8; PAGE_SIZE],
}

impl Page {
    // keep it internal
    pub(crate) fn new() -> Self {
        let init = [0u8; PAGE_SIZE];
        Page { data: init }
    }

    pub(crate) fn read_u8(&self, offset: usize) -> u8 {
        // 1 byte of integer
        self.data[offset]
    }

    pub(crate) fn read_u16(&self, offset: usize) -> u16 {
        // 2 bytes of integer
        let bytes: &[u8] = &self.data[offset..offset + 2];
        u16::from_le_bytes(bytes.try_into().unwrap())
    }

    pub(crate) fn read_u32(&self, offset: usize) -> u32 {
        // 4 bytes of integer
        let bytes: &[u8] = &self.data[offset..offset + 4];
        u32::from_le_bytes(bytes.try_into().unwrap())
    }

    pub(crate) fn read_u64(&self, offset: usize) -> u64 {
        // 8 bytes of integer
        let bytes = &self.data[offset..offset + 8];
        u64::from_le_bytes(bytes.try_into().unwrap())
    }

    pub(crate) fn read_bytes(&self, offset: usize, len: usize) -> &[u8] {
        &self.data[offset..offset + len]
    }

    pub(crate) fn write_u8(&mut self, offset: usize, value: u8) {
        self.data[offset] = value;
    }

    pub(crate) fn write_u16(&mut self, offset: usize, value: u16) {
        self.data[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
    }

    pub(crate) fn write_u32(&mut self, offset: usize, value: u32) {
        self.data[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }

    pub(crate) fn write_u64(&mut self, offset: usize, value: u64) {
        self.data[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
    }

    pub(crate) fn write_bytes(&mut self, offset: usize, src: &[u8]) {
        self.data[offset..offset + src.len()].copy_from_slice(src);
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.data
    }
    pub(crate) fn from_bytes(data: [u8; PAGE_SIZE]) -> Self {
        Self { data }
    }
}

impl Default for Page {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_page_is_zeroed() {
        let page = Page::new();
        assert_eq!(page.as_bytes(), &[0u8; PAGE_SIZE]);
    }

    #[test]
    fn u16_roundtrip() {
        let mut page = Page::new();
        page.write_u16(8, 42);
        page.write_u16(10, 99);
        assert_eq!(page.read_u16(8), 42);
        assert_eq!(page.read_u16(10), 99);
    }

    #[test]
    fn u32_roundtrip() {
        let mut page = Page::new();
        page.write_u32(8, 42);
        page.write_u32(12, 99);
        assert_eq!(page.read_u32(8), 42);
        assert_eq!(page.read_u32(12), 99);
    }

    #[test]
    fn u64_roundtrip() {
        let mut page = Page::new();
        page.write_u64(8, 0xDEADBEEFCAFEu64);
        page.write_u64(16, 0xDEADBEEFCAFEu64);
        assert_eq!(page.read_u64(8), 0xDEADBEEFCAFEu64);
        assert_eq!(page.read_u64(16), 0xDEADBEEFCAFEu64);
    }

    #[test]
    fn bytes_roundtrip() {
        let mut page = Page::new();
        let record: &[u8; 5] = b"hello";
        page.write_bytes(100, record);
        assert_eq!(page.read_bytes(100, 5), record);
    }

    #[test]
    fn overwriting_replaces_the_old_value() {
        let mut page = Page::new();
        page.write_u32(0, 42);
        page.write_u32(0, 7);
        assert_eq!(page.read_u32(0), 7);
    }

    #[test]
    fn page_id_offset() {
        assert_eq!(PageId(0).as_offset(), 0);
        assert_eq!(PageId(3).as_offset(), 3 * PAGE_SIZE as u64);
    }

    #[test]
    #[should_panic]
    fn reading_past_the_end() {
        let page = Page::new();
        page.read_u16(PAGE_SIZE - 1); // since 2 is needed
    }

    #[test]
    fn integers_are_stored_little_endian() {
        let mut page = Page::new();
        page.write_u32(0, 0x12345678);
        assert_eq!(page.read_bytes(0, 4), &[0x78, 0x56, 0x34, 0x12]);
        page.write_u64(4, 0x123456789ABCDEF0);
        assert_eq!(
            page.read_bytes(4, 8),
            &[0xF0, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12]
        );
    }
}
