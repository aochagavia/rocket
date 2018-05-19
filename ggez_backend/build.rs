use std::env;
use std::fs::{copy, create_dir_all, read_dir};
use std::path::{Path, PathBuf};
use std::io;

fn main() {
    let res_dir_source = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("../resources/");
    let res_dir_target = Path::new(&env::var("OUT_DIR").unwrap()).join("../../../resources/");

    //copies all resource files to "target/NAME/resources". Prints out any errors if failed.
    if let Err(io_error) = add_resources(&res_dir_source, &res_dir_target) {
        println!("OS Error: {}", io_error);
    }

    // Stuff for dealing with SDL
    let target = env::var("TARGET").unwrap();
    if target.contains("pc-windows") {
        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let mut lib_dir = manifest_dir.clone();
        let mut dll_dir = manifest_dir.clone();
        if target.contains("msvc") {
            lib_dir.push("msvc");
            dll_dir.push("msvc");
        } else {
            lib_dir.push("gnu-mingw");
            dll_dir.push("gnu-mingw");
        }
        lib_dir.push("lib");
        dll_dir.push("dll");
        if target.contains("x86_64") {
            lib_dir.push("64");
            dll_dir.push("64");
        } else {
            lib_dir.push("32");
            dll_dir.push("32");
        }
        println!("cargo:rustc-link-search=all={}", lib_dir.display());
        for entry in std::fs::read_dir(dll_dir).expect("Can't read DLL dir") {
            let entry_path = entry.expect("Invalid fs entry").path();
            let file_name_result = entry_path.file_name();
            let mut new_file_path = manifest_dir.clone();
            if let Some(file_name) = file_name_result {
                let file_name = file_name.to_str().unwrap();
                if file_name.ends_with(".dll") {
                    new_file_path.push(file_name);
                    std::fs::copy(&entry_path, new_file_path.as_path())
                        .expect("Can't copy from DLL dir");
                }
            }
        }
    }
}

///Recursively copy all files in dir given by source_path to dir given by target path
///WARNING! Overwrites files with same name
fn add_resources(source_path: &PathBuf, target_path: &PathBuf) -> io::Result<()> {
    match read_dir(source_path) {
        Ok(entry_iter) => {
            create_dir_all(target_path)?;
            for entry in entry_iter {
                let entry = entry?;
                let source_path = entry.path();
                let target_path = target_path.join(entry.file_name());
                add_resources(&source_path, &target_path)?;
            }
        }
        Err(_) => {
            copy(&source_path, &target_path)?;
        }
    }
    Ok(())
}
