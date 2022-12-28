pub mod error;
pub mod generated;
pub mod parser;
pub mod structs;

fn main() {
    let file = match parser::ManifestFile::try_from_path("2B9C4AABEFC317D2.manifest") {
        Ok(result) => result,
        Err(error) => panic!("{:?}", error),
    };

    for files in file.manifest.files {
        // print!("name: {}; ", files.name);
        // print!("path: {}; ", files.path);
        // print!("size: {} bytes; ", files.size);
        // print!("langs: {:?}; ", files.languages);
        // println!("");
    }
}
