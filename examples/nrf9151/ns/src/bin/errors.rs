/// Crate error types
#[derive(Debug)]
pub enum Error {
    NrfModem(nrf_modem::Error),
    Timeout(embassy_time::TimeoutError),
    ParseError(at_commands::parser::ParseError),
}

impl From<nrf_modem::Error> for Error {
    fn from(e: nrf_modem::Error) -> Self {
        Self::NrfModem(e)
    }
}

impl From<embassy_time::TimeoutError> for Error {
    fn from(e: embassy_time::TimeoutError) -> Self {
        Self::Timeout(e)
    }
}

impl From<at_commands::parser::ParseError> for Error {
    fn from(e: at_commands::parser::ParseError) -> Self {
        Self::ParseError(e)
    }
}
