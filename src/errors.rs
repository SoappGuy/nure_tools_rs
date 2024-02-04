use thiserror::Error;

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("Can't get any response")]
    GetFailed,

    #[error("Got respond not in json format")]
    NotJson,

    #[error("API returned data in unexpexted format")]
    InvalidReturn,

    #[error("API returned with statuscode: {0} - {1}")]
    BadResponse(String, u16),
}

#[derive(Debug, Error)]
pub enum FindError {
    #[error("Can't find group with name: {0}")]
    InvalidGroupName(String),

    #[error("Can't find lecture room with name: {0}")]
    InvalidLectureRoomName(String),

    #[error("Can't find teacher with name: {0}")]
    InvalidTeacherName(String),

    #[error("Can't compile Regex from given string: {0}")]
    InvalidRegexString(String),
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Can't parse DateTime from string: {0}")]
    InvalidStringProvided(String),

    #[error("Can't parse DateTime from timestamp: {0}")]
    InvalidTimestampProvided(String),
}
