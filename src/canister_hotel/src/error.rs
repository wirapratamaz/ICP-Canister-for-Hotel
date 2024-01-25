#[derive(candid::CandidType, candid::Deserialize, serde::Serialize)]
pub enum Error {
    RoomFull,
    RoomAlreadyBooked,
    RoomNotBooked,
    InvalidUpdate,
}