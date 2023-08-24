use super::method::{Method, MethodError};
use std::error::Error;
use std::fmt::{Result as FmtResult, Display, Formatter, Debug};
use std::str::{self, Utf8Error};

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

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;

        match get_next_word(request) {
            Some( (method, request)) => {},
            None => return Err(ParseError::InvalidRequest),
        }

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        return {
            Ok(Self {
                path: path.to_string(),
                query_string: None,
                method: method.parse()?,
            })
        }
    }

}


impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {

    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some( (&request[..i], &request[i+1..]) );
        }
    }

    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    } 
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