use crate::mc::proto::types::VarInt;
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
    }
};

pub struct Handshake {
    pub proto_ver: VarInt,
    pub server_addr: String,
    pub server_port: u16,
    pub next_state: VarInt,
}

impl PacketId for Handshake {
    fn id(&self) -> VarInt {
        VarInt(0x0)
    }
}

#[async_trait]
impl Serialize for Handshake {
    async fn decode<R>(buf: &mut R) -> Result<Self, ScanError> where R: Read + Unpin + Send {
        Ok(Self {
            proto_ver: VarInt::decode(buf).await?,
            server_addr: String::decode(buf).await?,
            server_port: u16::decode(buf).await?,
            next_state: VarInt::decode(buf).await?,
        })
    }

    async fn encode<W>(&self, buf: &mut W) -> Result<(), ScanError> where W: Write + Unpin + Send {
        self.proto_ver.encode(buf).await?;
        self.server_addr.encode(buf).await?;
        self.server_port.encode(buf).await?;
        self.next_state.encode(buf).await?;
        Ok(())
    }
}