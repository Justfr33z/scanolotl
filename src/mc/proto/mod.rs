use crate::mc::proto::types::VarInt;

mod ser;

pub use ser::Serialize;

pub mod types;
pub mod packets;

pub trait PacketId {
    fn id(&self) -> VarInt;
}