use std::borrow;
use candid::{Encode, Decode };

use crate::error;

pub struct Occupant {
    pub id: String,
    pub start_date: u64,
    pub end_date: u64,
}

impl Occupant {
    pub fn new(id: String, start_date: u64, end_date: u64) -> Self {
        Self {
            id: id.to_string(),
            start_date,
            end_date,
        }
    }
}

pub enum RoomState {
    Full,
    PartiallyOccupied,
    TotallyVacant,
}

pub struct Room {
    pub no: u64,
    pub state: RoomState,
    pub occupants: Vec<Occupant>,
    pub price_per_occupant: u64,
    pub capacity: u64,
    pub owner: Occupant
}

impl Room {
    pub fn new(number: u64, capacity: u64, price_per_occupant: u64, owner: Occupant) -> Self {
        Self {
            Room {
                no: number,
                state: RoomState::TotallyVacant,
                occupants: Vec::new(),
                price_per_occupant,
                capacity,
                owner
            }
        }
    }

    pub fn add_occupant(&mut self, occupant: Occupant) -> Result<(), error::Error> {
        is self.is_full() {
            return Err(error::Error::RoomFull);
        }

        match self.has_occupant(occupant.clone()) {
            Some(_)=> Err(error::Error::RoomAlreadyBooked),
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

    pub fn remove_occupant(&mut, self, occupant:Occupant) -> Result<(), error::Error> {
        match self.has_occupant(occupant){
            Some(index) => {
                self.occupants.remove(index);
                self.state = if self.occupants.len() == 0 {
                    RoomState::TotallyVacant
                } else {
                    RoomState::PartiallyOccupied
                };
                Ok(())
            }
            None => Err(error::Error::RoomNotBooked),
        }
    }

    pub fn update_occupant(&mut, self, new_capacity:Option<u64>, new_price_per_occupant:Option<u64>) -> Result<(), error::Error> {
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
}