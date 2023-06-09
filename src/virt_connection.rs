use std::env;
use anyhow::Result;
use virt::connect::Connect;

pub struct VirtConnection {
    uri: String,
}

impl VirtConnection {
    pub fn new() -> Result<Self> {
        let uri = env::args().next().unwrap_or_default();
        return Ok(VirtConnection { uri });
    }

    pub fn connect(&mut self) -> Result<Connect> {
        let connection = Connect::open(&self.uri)?;
        return Ok(connection);
    }

    pub fn disconnect(mut connection: Connect) -> Result<i32> {
        return Ok(connection.close()?);
    }
}

