use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

pub const KEY: &str = include_str!("../key.txt");

#[derive(Serialize, Deserialize, Default, EnumIter, Display, Debug)]
pub enum PacketBody {
    Message(Message),
    #[default]
    Ping,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Message(pub String);

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Packet<'a, T = Unvalidated> {
    pub body: PacketBody,
    pub key: &'a str,
    valid: PhantomData<T>,
}

pub struct Validated;
pub struct Unvalidated;

impl Packet<'_, Validated> {
    pub fn new(body: PacketBody) -> Self {
        Self {
            body,
            key: KEY,
            valid: PhantomData,
        }
    }
}

impl<'a> Packet<'a, Unvalidated> {
    pub fn validate(self) -> Option<Packet<'a, Validated>> {
        if self.key == KEY {
            Some(Packet::<'a, Validated> {
                body: self.body,
                key: self.key,
                valid: PhantomData,
            })
        } else {
            None
        }
    }
}

impl PacketBody {
    pub fn new_packet<'a>(self) -> Packet<'a, Validated> {
        Packet::new(self)
    }
}
