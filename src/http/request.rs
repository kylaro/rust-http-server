use super::method::Method;
use std::error::Error;
use std::fmt::{Result as FmtResult, Display, Formatter, Debug};

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl Request {
    fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
        unimplemented!();
    }
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!();
    }

}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl ParseError {
    fn message(&self) -> &str{
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}


/* Request
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/

/* Response
HTTP/1.1 200 OK\r\n
HEADERS \r\n
BODY 
*/