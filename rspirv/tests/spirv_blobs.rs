use rspirv::{
    binary::{Assemble as _, Disassemble as _},
    dr,
};

use std::path::PathBuf;

fn test_external_dir(dir_path: PathBuf) {
    use std::fs;

    let dir_iter = match fs::read_dir(dir_path) {
        Ok(dir) => dir,
        Err(_) => return,
    };
    for entry in dir_iter {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let fty = entry.file_type().unwrap();
        let path = entry.path();
        if fty.is_file() {
            match path.extension() {
                Some(ext) => {
                    if ext.to_string_lossy() != "spv" {
                        continue
                    }
                }
                None => continue
            }
            let spv = fs::read(path).unwrap();
            let module = dr::load_bytes(spv).unwrap();
            let _disasm = module.disassemble();
            let _assembly = module.assemble();
        } else {
            test_external_dir(path);
        }
    }
}

#[test]
fn test_external_modules() {
    let dir_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("spirv-blobs");
    test_external_dir(dir_path);
}
