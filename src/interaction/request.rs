#[repr(u8)]
pub enum RequestType {
    Ping = 1,
    ApplicationCommand = 2
}

impl TryFrom<u8> for RequestType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(RequestType::Ping),
            2 => Ok(RequestType::ApplicationCommand),
            _ => Err(()),
        }
    }
}
