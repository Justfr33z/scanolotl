use async_trait::async_trait;
use async_std::io::prelude::{
    Read,
    Write,
};
use crate::{
    scanner::ScanError,
    mc::proto::{
        PacketId,
        Serialize,
        types::VarInt,
    },
};

pub struct Response {
    pub json: String,
}

impl PacketId for Response {
    fn id(&self) -> VarInt {
        VarInt(0x0)
    }
}

#[async_trait]
impl Serialize for Response {
    async fn decode<R>(buf: &mut R) -> Result<Self, ScanError> where R: Read + Unpin + Send {
        Ok(Self {
            json: String::decode(buf).await?,
        })
    }

    async fn encode<W>(&self, buf: &mut W) -> Result<(), ScanError> where W: Write + Unpin + Send {
        self.json.encode(buf).await?;
        Ok(())
    }
}