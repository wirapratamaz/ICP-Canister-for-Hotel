use ic_stable_structures::memory_manager;
use std::collections::HashMap;
use std::cell;
use std::f32::consts::E;

// Import custom error and model modules that define the application's data structures and error types.
mod error;
mod model;

// Define thread-local storage for the memory manager and rooms.
// This allows each thread to have its own instances of these variables.
thread_local! {
    static MEMORY_MANAGER: cell::RefCell<memory_manager::MemoryManager<ic_stable_structures::DefaultMemoryImpl>> = cell::RefCell::new(memory_manager::MemoryManager::init(ic_stable_structures::DefaultMemoryImpl::default()));

    // A HashMap to store rooms, using a unique u64 identifier for each room.
    static ROOMS: cell::RefCell<HashMap<u64, model::Room>> = cell::RefCell::new(
        HashMap::new()
    );
}

// Define a query method to retrieve a room by its ID.
#[ic_cdk::query]
fn get_room(room_id: u64) -> model::Room {
    ROOMS.with(|rooms| {
        rooms.borrow().get(&room_id).unwrap().clone()
    })
}

// Define a query method to retrieve a room by its number.
#[ic_cdk::query]
fn get_room_by_number(
    payload: model::GetRoomByNumberPayload,
) -> Result<model::Room, error::Error> {
    ROOMS.with(|r| {
        let rooms = r.borrow();
        let room = rooms
            .get(&payload.number)
            .ok_or(error::Error::RoomNotFound)?;
        Ok(room.clone())
    })
}

// Define an update method to create a new room.
#[ic_cdk::update]
fn create_room(payload: model::CreateRoomPayload) -> Result<String, error::Error> {
    // Convert the Principal to a String for the id
    let caller_str = ic_cdk::caller().to_string();

    // Use the current time from the IC for start_date and end_date
    let current_time: u64 = ic_cdk::api::time();

    // For example, to set an end_date 24 hours from now, you can add 24 hours worth of nanoseconds:
    // 24 hours * 60 minutes * 60 seconds * 1_000_000_000 nanoseconds
    let one_day_in_nanoseconds: u64 = 24 * 60 * 60 * 1_000_000_000;
    let end_date: u64 = current_time + one_day_in_nanoseconds;

    // Create a new Room with the correct arguments
    let room = model::Room::new(
        payload.number,
        payload.capacity,
        payload.price_per_occupant,
        model::Occupant::new(caller_str, current_time, end_date) // Pass the correct arguments here
    );

    ROOMS.with(|r| {
        let mut rooms = r.borrow_mut();
        if rooms.contains_key(&room.no) {
            return Err(error::Error::RoomAlreadyExists);
        }
        rooms.insert(room.no, room);
        Ok(String::from("Room created successfully!"))
    })
}

// Define an update method to book a room.
#[ic_cdk::update]
fn book_room(payload: model::BookRoomPayload) -> Result<String, error::Error> {
    // Convert the Principal to a String for the id
    let caller_str = ic_cdk::caller().to_string();

    // Use the current time from the IC for start_date and end_date
    let current_time: u64 = ic_cdk::api::time();

    // For example, to set an end_date 24 hours from now, you can add 24 hours worth of nanoseconds:
    // 24 hours * 60 minutes * 60 seconds * 1_000_000_000 nanoseconds
    let one_day_in_nanoseconds: u64 = 24 * 60 * 60 * 1_000_000_000;
    let end_date: u64 = current_time + one_day_in_nanoseconds;

    ROOMS.with(|r| {
        let mut rooms = r.borrow_mut();
        let room = rooms
            .get_mut(&payload.number)
            .ok_or(error::Error::RoomNotFound)?;

        if room.is_full() {
            return Err(error::Error::RoomFull);
        }

        if !room.price_check(payload.price) {
            return Err(error::Error::InvalidPrice);
        }

        let occupant = model::Occupant::new(caller_str, current_time, end_date);// Pass the correct arguments hereer());

        match room.add_occupant(occupant) {
            Ok(_) => Ok(String::from("Room successfully booked")),
            Err(err) => Err(err),
        }
    })
}

// Define an update method to unbook a room.
#[ic_cdk::update]
fn unbook_room(payload: model::UnbookRoomPayload) -> Result<String, error::Error> {
    // Convert the Principal to a String for the id
    let caller_str = ic_cdk::caller().to_string();

    // Use the current time from the IC for start_date and end_date
    let current_time: u64 = ic_cdk::api::time();

    // For example, to set an end_date 24 hours from now, you can add 24 hours worth of nanoseconds:
    // 24 hours * 60 minutes * 60 seconds * 1_000_000_000 nanoseconds
    let one_day_in_nanoseconds: u64 = 24 * 60 * 60 * 1_000_000_000;
    let end_date: u64 = current_time + one_day_in_nanoseconds;
    
    ROOMS.with(|r| {
        let mut rooms = r.borrow_mut();
        let room = rooms
            .get_mut(&payload.number)
            .ok_or(error::Error::RoomNotFound)?;
        let occupant = model::Occupant::new(caller_str, current_time, end_date);

        // Pass a reference to the occupant instead of an owned value
        match room.has_occupant(&occupant) {
            Some(_) => match room.remove_occupant(&occupant) {
                Ok(_) => Ok(String::from("Room unbooked successfully!")),
                Err(_) => Err(error::Error::NotInRoom),
            },
            None => Err(error::Error::NotInRoom),
        }
    })
}

// Define an update method to delete a room.
#[ic_cdk::update]
fn delete_room(payload: model::DeleteRoomPayload) -> Result<(), error::Error> {
    ROOMS.with(|r| {
        let mut rooms = r.borrow_mut();
        rooms.remove(&payload.number);
        Ok(())
    })
}