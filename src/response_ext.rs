use std::io::Read;

pub trait ResponseExt {
    fn read_bytes_with_limit(self, limit: usize) -> anyhow::Result<Vec<u8>>;
}

impl ResponseExt for ureq::Response {
    fn read_bytes_with_limit(self, limit: usize) -> anyhow::Result<Vec<u8>> {
        let mut bytes: Vec<u8> = Vec::with_capacity(128 * 1024);
        self.into_reader()
            .take(limit as u64)
            .read_to_end(&mut bytes)?;
        Ok(bytes)
    }
}