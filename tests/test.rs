use rman_rs::{Header, RiotManifest};

mod common;

#[test]
pub fn should_parse_from_path_when_valid_manifest() {
    let valid_file = common::ValidManifest::new();
    valid_file.generate();
    let mut path = valid_file.path();
    path.set_extension("manifest");
    if let Err(error) = RiotManifest::from_path(path) {
        panic!(
            "there was an error when trying to parse the manifest, manifest: {:?}",
            error
        );
    };
}

#[test]
pub fn should_have_correct_values_when_valid_manifest() {
    let valid_file = common::ValidManifest::new();
    valid_file.generate();
    let mut path = valid_file.path();
    path.set_extension("manifest");
    let manifest = RiotManifest::from_path(path).unwrap();

    // FIXME: don't check for equality on compressed size and uncompressed size
    // compressed and uncompressed size could change in the future
    let header = Header {
        magic: 0x4E414D52,
        major: 2,
        minor: 0,
        flags: 512,
        offset: 28,
        compressed_size: 189,
        manifest_id: 0,
        uncompressed_size: 376,
    };

    assert_eq!(
        &manifest.header, &header,
        "manifest header should be the same"
    );
    assert_eq!(
        manifest.data.bundle_entries.len(),
        1,
        "should have 1 bundle entry"
    );
    assert_eq!(
        manifest.data.bundle_entries[0].chunks.len(),
        1,
        "bundle entry should have 1 chunk"
    );
    assert_eq!(
        manifest.data.directory_entries.len(),
        2,
        "should have 2 directory entries"
    );
    assert_eq!(
        manifest.data.file_entries.len(),
        1,
        "should have 1 file entry"
    );
    assert_eq!(manifest.data.files.len(), 1, "should parse into 1 file");
    assert_eq!(
        manifest.data.key_entries.len(),
        1,
        "should have 1 key entry"
    );
    assert_eq!(
        manifest.data.language_entries.len(),
        1,
        "should have 1 language entries"
    );
    assert_eq!(
        manifest.data.param_entries.len(),
        1,
        "should have 1 param entry"
    );
}

#[test]
pub fn should_parse_from_path_when_valid_empty_manifest() {
    let valid_empty_file = common::ValidEmptyManifest::new();
    valid_empty_file.generate();
    let mut path = valid_empty_file.path();
    path.set_extension("manifest");
    if let Err(error) = RiotManifest::from_path(path) {
        panic!(
            "there was an error when trying to parse the manifest, manifest: {:?}",
            error
        );
    };
}

#[test]
pub fn should_have_correct_values_when_valid_empty_manifest() {
    let valid_empty_file = common::ValidEmptyManifest::new();
    valid_empty_file.generate();
    let mut path = valid_empty_file.path();
    path.set_extension("manifest");
    let manifest = RiotManifest::from_path(path).unwrap();

    // FIXME: don't check for equality on compressed size and uncompressed size
    // compressed and uncompressed size could change in the future
    let header = Header {
        magic: 0x4E414D52,
        major: 2,
        minor: 0,
        flags: 512,
        offset: 28,
        compressed_size: 59,
        manifest_id: 0,
        uncompressed_size: 72,
    };

    assert_eq!(
        &manifest.header, &header,
        "manifest header should be the same"
    );
    assert_eq!(
        manifest.data.bundle_entries.len(),
        0,
        "should have 0 bundle entries"
    );
    assert_eq!(
        manifest.data.directory_entries.len(),
        0,
        "should have 0 directory entries"
    );
    assert_eq!(
        manifest.data.file_entries.len(),
        0,
        "should have 0 file entries"
    );
    assert_eq!(manifest.data.files.len(), 0, "should parse into 0 files");
    assert_eq!(
        manifest.data.key_entries.len(),
        0,
        "should have 0 key entries"
    );
    assert_eq!(
        manifest.data.language_entries.len(),
        0,
        "should have 0 language entries"
    );
    assert_eq!(
        manifest.data.param_entries.len(),
        0,
        "should have 0 param entries"
    );
}
