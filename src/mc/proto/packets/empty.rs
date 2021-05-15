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

pub struct Empty;

impl PacketId for Empty {
    fn id(&self) -> VarInt {
        VarInt(0x0)
    }
}

#[async_trait]
impl Serialize for Empty {
    async fn decode<R>(_: &mut R) -> Result<Self, ScanError> where R: Read + Unpin + Send {
        Ok(Self)
    }

    async fn encode<W>(&self, _: &mut W) -> Result<(), ScanError> where W: Write + Unpin + Send {
        Ok(())
    }
}