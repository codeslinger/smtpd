// Copyright (c) 2014 Toby DiPasquale <toby@cbcg.net>
use std::error::FromError;
use std::fmt::Show;
use std::fmt;
use std::io;

pub enum SError {
    Io(io::IoError),
    Parse(ParseError),
}

impl Show for SError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SError::Io(ref x) => { x.fmt(f) }
            SError::Parse(ref x) => { x.fmt(f) }
        }
    }
}

impl FromError<io::IoError> for SError {
    fn from_error(err: io::IoError) -> SError { SError::Io(err) }
}

impl FromError<ParseError> for SError {
    fn from_error(err: ParseError) -> SError { SError::Parse(err) }
}

#[deriving(Copy, Send, PartialEq)]
pub enum ParseError {
    InvalidLineFormat(uint),
    InvalidArgument(uint),
}

impl Show for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::InvalidLineFormat(line) => { write!(f, "line {}: missing colon delimiter", line) }
            ParseError::InvalidArgument(line) => { write!(f, "line {}: invalid argument", line) }
        }
    }
}

