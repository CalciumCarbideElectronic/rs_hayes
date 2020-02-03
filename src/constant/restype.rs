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
