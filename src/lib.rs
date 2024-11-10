use std::path::Path;

mod errors;
mod data;

pub fn read_map(path: impl AsRef<Path>) -> Result<data::Tilemap, errors::TiledParserError> {
	let path_ref = path.as_ref();
	Err(errors::NotSupportedError::new("Not implemented").into())
}

pub fn read_tileset(path: impl AsRef<Path>) -> Result<data::Tileset, errors::TiledParserError> {
	let path_ref = path.as_ref();
	Err(errors::NotSupportedError::new("Not implemented").into())
}


pub fn write_map(tilemap: &data::Tilemap, path: impl AsRef<Path>) -> Result<(), errors::TiledParserError> {
	let path_ref = path.as_ref();
	Err(errors::NotSupportedError::new("Not implemented").into())
}

pub fn write_tileset(tileset: &data::Tileset, path: impl AsRef<Path>) -> Result<(), errors::TiledParserError> {
	let path_ref = path.as_ref();
	Err(errors::NotSupportedError::new("Not implemented").into())
}
