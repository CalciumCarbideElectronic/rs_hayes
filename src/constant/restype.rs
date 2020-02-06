use alloc::string::String;
use serde::Deserialize;

#[derive(PartialEq, Deserialize)]
pub struct CGATTResponse {
    pub state: String,
}

#[derive(PartialEq)]
pub enum CGATT_STATE {
    Detached = 0,
    Attached = 1,
}

#[derive(PartialEq)]
pub enum CSCON_STATE {
    EnableURC = 0,
    Attached = 1,
}

#[derive(PartialEq)]
pub enum QMTOPENState {
    FailedToOpen = -1,
    Ok = 0,
    WrongParamater = 1,
    MQTTIdentifierIsOccupied = 2,
    FailedToActivatePDP = 3,
    FailedToParseDomain = 4,
    NetworkDisconnect = 5,
}

#[derive(PartialEq)]
pub struct QMTOPENReadResponse {
    conn_id: u8,
    hostname: String,
    port: u16,
}
#[derive(PartialEq)]
pub struct QMTOPENWriteResponse {
    conn_id: u8,
    result: QMTOPENState,
}

#[derive(Deserialize,Debug)]
pub struct QMTPUBResponse {
    conn_id: String,
    msg_id: String,
    result: String,
}
