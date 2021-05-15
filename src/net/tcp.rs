use async_trait::async_trait;
use futures::{
    AsyncReadExt,
    AsyncWriteExt,
};
use async_std::{
    io::Cursor,
    net::TcpStream,
};
use crate::{
    scanner::ScanError,
    mc::proto::{
        PacketId,
        Serialize,
        types::VarInt,
    },
};

#[async_trait]
pub trait PacketEncoding {
    async fn read_packet(&mut self) -> Result<(i32, Cursor<Vec<u8>>), ScanError>;
    async fn write_packet<T>(&mut self, packet: T) -> Result<(), ScanError> where T: Serialize + PacketId + Send + Sync;
}

#[async_trait]
impl PacketEncoding for TcpStream {
    async fn read_packet(&mut self) -> Result<(i32, Cursor<Vec<u8>>), ScanError> {
        let mut buf = vec![0; VarInt::decode(self).await?.0 as usize];
        self.read_exact(&mut buf).await?;

        let mut cur = Cursor::new(buf);
        Ok((VarInt::decode(&mut cur).await?.0, cur))
    }

    async fn write_packet<T>(&mut self, packet: T) -> Result<(), ScanError> where T: Serialize + PacketId + Send + Sync {
        let mut buf = Vec::new();
        let mut data_buf = Vec::new();

        packet.id().encode(&mut data_buf).await?;
        packet.encode(&mut data_buf).await?;

        VarInt(data_buf.len() as i32).encode(&mut buf).await?;
        buf.append(&mut data_buf);

        self.write_all(&buf).await?;
        Ok(())
    }
}