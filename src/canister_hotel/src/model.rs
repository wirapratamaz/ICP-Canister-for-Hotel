use std::borrow;
use candid::{Encode, Decode};

use crate::error;

#[derive(candid::CandidType, candid::Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Occupant {
    pub id: String,
    pub start_date: u64,
    pub end_date: u64,
}

impl Occupant {
    pub fn new(id: String, start_date: u64, end_date: u64) -> Self {
        Self {
            id,
            start_date,
            end_date,
        }
    }
}

#[derive(candid::CandidType, candid::Deserialize, Clone, PartialEq, Eq, Debug)]
pub enum RoomState {
    Full,
    PartiallyOccupied,
    TotallyVacant,
}

#[derive(candid::CandidType, candid::Deserialize, Clone, Debug)]
pub struct Room {
    pub no: u64,
    pub state: RoomState,
    pub occupants: Vec<Occupant>,
    pub price_per_occupant: u64,
    pub capacity: u64,
    pub owner: Occupant,
}

impl Room {
    pub fn new(no: u64, capacity: u64, price_per_occupant: u64, owner: Occupant) -> Self {
        Self {
            no,
            state: RoomState::TotallyVacant,
            occupants: Vec::new(),
            price_per_occupant,
            capacity,
            owner,
        }
    }

    pub fn add_occupant(&mut self, occupant: Occupant) -> Result<(), error::Error> {
        if self.is_full() {
            return Err(error::Error::RoomFull);
        }

        match self.has_occupant(&occupant) {
            Some(_) => Err(error::Error::RoomAlreadyBooked),
            None => {
                self.occupants.push(occupant);
                self.state = if self.occupants.len() == self.capacity as usize {
                    RoomState::Full
                } else {
                    RoomState::PartiallyOccupied
                };
                Ok(())
            }
        }
    }

    pub fn remove_occupant(&mut self, occupant: &Occupant) -> Result<(), error::Error> {
        match self.has_occupant(occupant) {
            Some(index) => {
                self.occupants.remove(index);
                self.state = if self.occupants.is_empty() {
                    RoomState::TotallyVacant
                } else {
                    RoomState::PartiallyOccupied
                };
                Ok(())
            }
            None => Err(error::Error::RoomNotBooked),
        }
    }

    pub fn update_occupant(&mut self, new_capacity: Option<u64>, new_price_per_occupant: Option<u64>) -> Result<(), error::Error> {
        if let Some(capacity) = new_capacity {
            if capacity < self.occupants.len() as u64 {
                return Err(error::Error::InvalidUpdate);
            }
            self.capacity = capacity;
        }
        if let Some(price) = new_price_per_occupant {
            self.price_per_occupant = price;
        }
        Ok(())
    }

    pub fn price_check(&self, price: u64) -> bool {
        price == self.price_per_occupant
    }

    pub fn is_full(&self) -> bool {
        self.state == RoomState::Full
    }

    pub fn has_occupant(&self, occupant: &Occupant) -> Option<usize> {
        self.occupants.iter().position(|o| o.id == occupant.id)
    }

    pub fn is_owner(&self, occupant: &Occupant) -> bool {
        self.owner == *occupant
    }
}

impl ic_stable_structures::Storable for Room {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        borrow::Cow::Owned(Encode!(&self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Room {
        Decode!(&bytes, Room).unwrap()
    }
}

impl ic_stable_structures::BoundedStorable for Room {
    const MAX_SIZE: u32 = 1000;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(candid::CandidType, candid::Deserialize)]
pub struct GetRoomByNumberPayload {
    pub number: u64,
}

#[derive(candid::CandidType, candid::Deserialize)]
pub struct CreateRoomPayload {
    pub number: u64,
    pub capacity: u64,
    pub price_per_occupant: u64,
}

#[derive(candid::CandidType, candid::Deserialize)]
pub struct BookRoomPayload {
    pub number: u64,
    pub price: u64,
}

#[derive(candid::CandidType, candid::Deserialize)]
pub struct UnbookRoomPayload {
    pub number: u64,
}

#[derive(candid::CandidType, candid::Deserialize)]
pub struct UpdateRoomPayload {
    pub number: u64,
    pub capacity: Option<u64>,
    pub price_per_occupant: Option<u64>,
}

#[derive(candid::CandidType, candid::Deserialize)]
pub struct DeleteRoomPayload {
    pub number: u64,
}