use async_trait::async_trait;
use futures::{
    AsyncReadExt,
    AsyncWriteExt,
};
use async_std::io::prelude::{
    Read,
    Write,
};
use crate::{
    scanner::ScanError,
    mc::proto::types::VarInt,
};
use byteorder::{
    BigEndian,
    ReadBytesExt,
    WriteBytesExt,
};

#[async_trait]
pub trait Serialize : Sized {
    async fn decode<R>(buf: &mut R) -> Result<Self, ScanError> where R: Read + Unpin + Send;
    async fn encode<W>(&self, buf: &mut W) -> Result<(), ScanError> where W: Write + Unpin + Send;
}

#[async_trait]
impl Serialize for u16 {
    async fn decode<R>(buf: &mut R) -> Result<Self, ScanError> where R: Read + Unpin + Send {
        Ok(buf.read_u16::<BigEndian>().await?)
    }

    async fn encode<W>(&self, buf: &mut W) -> Result<(), ScanError> where W: Write + Unpin + Send {
        buf.write_u16::<BigEndian>(*self).await?;
        Ok(())
    }
}

#[async_trait]
impl Serialize for VarInt {
    async fn decode<R>(buf: &mut R) -> Result<Self, ScanError> where R: Read + Unpin + Send {
        let mut index = 0;
        let mut result = 0;

        loop {
            let read = buf.read_u8().await?;
            let val = (read & 0b01111111) as i32;
            result |= val << (7 * index);

            index += 1;
            if index > 5 {
                return Err(ScanError::Error("VarInt is to big".to_owned()));
            }

            if (read & 0b10000000) == 0 {
                break;
            }
        }

        Ok(VarInt(result))
    }

    async fn encode<W>(&self, buf: &mut W) -> Result<(), ScanError> where W: Write + Unpin + Send {
        let mut val = self.0 as u32;

        loop {
            let mut temp = (val & 0b01111111) as u8;

            val >>= 7;
            if val != 0 {
                temp |= 0b10000000;
            }

            buf.write_u8(temp).await?;

            if val == 0 {
                break;
            }
        }

        Ok(())
    }
}

#[async_trait]
impl Serialize for String {
    async fn decode<R>(buf: &mut R) -> Result<Self, ScanError> where R: Read + Unpin + Send {
        let len = VarInt::decode(buf).await?.0;

        if len < 0 {
            return Err(ScanError::Error("Encoded string is empty".to_owned()));
        } else if len > 32767 * 4 {
            return Err(ScanError::Error("Encoded string is to long".to_owned()));
        }

        let mut bytes: Vec<u8> = Vec::with_capacity(len as usize);
        buf.take(len as u64).read_to_end(&mut bytes).await?;

        let str = String::from_utf8(bytes)?;
        if str.len() > 32767 {
            return Err(ScanError::Error("String is to long".to_owned()))
        }

        Ok(str)
    }

    async fn encode<W>(&self, buf: &mut W) -> Result<(), ScanError> where W: Write + Unpin + Send {
        let bytes = self.as_bytes();
        VarInt(bytes.len() as i32).encode(buf).await?;
        buf.write_all(bytes).await?;
        Ok(())
    }
}