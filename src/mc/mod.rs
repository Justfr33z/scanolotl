use serde::{
    Serialize,
    Deserialize,
};

pub mod proto;

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    pub ip: String,
    pub info: Information,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Information {
    pub version: Version,
    pub players: Players,
    pub description: Description,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    pub name: String,
    pub protocol: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Players {
    pub max: i32,
    pub online: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Description {
    pub text: String,
}