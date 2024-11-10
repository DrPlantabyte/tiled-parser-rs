/*!
The tiled_parser::error module holds an enum of possible error types, each of which
has a corresponding implementation struct.
*/

use std::fmt::{Debug, Display, Formatter};
use kiss_xml;

/// Represents an error that occurs during parsing or processing of XML
#[derive(Debug)]
pub enum TiledParserError {
	/// Error returned when the file is not recognized as a Tiled map or tileset
	InvalidData(InvalidData),
	/// Error indicating an attempt to do something that is not (yet) supported
	NotSupportedError(NotSupportedError),
	/// Error returned when there is a problem with the XML format or syntax
	XmlError(kiss_xml::errors::KissXmlError)
}

impl std::error::Error for TiledParserError{}

impl From<kiss_xml::errors::KissXmlError> for TiledParserError {
	fn from(e: kiss_xml::errors::KissXmlError) -> Self {TiledParserError::XmlError(e)}
}

impl Display for TiledParserError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			TiledParserError::InvalidData(e) => write!(f, "{}", e),
			TiledParserError::NotSupportedError(e) => write!(f, "{}", e),
			TiledParserError::XmlError(e) => write!(f, "{}", e),
		}
	}
}


/// Error indicating an attempt to create a comment or CDATA with invalid content (eg a comment with "-->" inside)
#[derive(Clone, Debug)]
pub struct InvalidData {
	/// The error message.
	pub msg: String
}

impl InvalidData{
	/// New error with a given message
	pub fn new(msg: impl Into<String>) -> Self {
		Self{msg: msg.into()}
	}
}

impl From<InvalidData> for TiledParserError {
	fn from(e: InvalidData) -> Self {TiledParserError::InvalidData(e)}
}

impl Display for InvalidData {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "InvalidData: {}", self.msg)
	}
}

impl std::error::Error for InvalidData{}


/// Error indicating an attempt to do something that is not (yet) supported
#[derive(Clone, Debug)]
pub struct NotSupportedError {
	/// The error message.
	pub msg: String
}

impl NotSupportedError{
	/// New error with a given message
	pub fn new(msg: impl Into<String>) -> Self {
		Self{msg: msg.into()}
	}
}

impl From<NotSupportedError> for TiledParserError {
	fn from(e: NotSupportedError) -> Self {TiledParserError::NotSupportedError(e)}
}

impl Display for NotSupportedError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "NotSupportedError: {}", self.msg)
	}
}

impl std::error::Error for NotSupportedError{}
