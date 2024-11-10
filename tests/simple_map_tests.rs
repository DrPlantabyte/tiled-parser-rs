
#[test]
fn test_read_write_simple() {
	use tiled_parser;
	use std::path::PathBuf;
	use tempfile;
	let tmp_dir = tempfile::tempdir().unwrap();
	let test_data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/simple-tileset");
	// round-trip test with .tsx file
	let tset = tiled_parser::read_tileset(test_data_dir.join("simple.tsx")).unwrap();
	let tmp_tsx_file = tmp_dir.path().join("simple-out.tsx");
	tiled_parser::write_tileset(&tset, &tmp_tsx_file).unwrap();
	let tset2 = tiled_parser::read_tileset(&tmp_tsx_file).unwrap();
	assert_eq!(tset, tset2, "simple.tsx round-trip test failed. {:?} != {:?}", tset, tset2);
	assert!(false, "Test implementation not finished yet");
}
