#![allow(dead_code, unused)]

#[path = "./src/generated/flatbuffer.rs"]
mod flatbuffer;

use std::fs;
use std::io::{Error, Write};

use flatbuffer::rman::{Bundle, BundleArgs};
use flatbuffer::rman::{Chunk, ChunkArgs};
use flatbuffer::rman::{Directory, DirectoryArgs};
use flatbuffer::rman::{File, FileArgs};
use flatbuffer::rman::{Key, KeyArgs};
use flatbuffer::rman::{Language, LanguageArgs};
use flatbuffer::rman::{Manifest, ManifestArgs};
use flatbuffer::rman::{Param, ParamArgs};
use flatbuffers::{FlatBufferBuilder, ForwardsUOffset, Vector, WIPOffset};

fn make_header(compressed_size: u32, uncompressed_size: u32) -> Vec<u8> {
    const MAGIC: [u8; 4] = [b'R', b'M', b'A', b'N'];
    const MAJOR: [u8; 1] = [2];
    const MINOR: [u8; 1] = [0];
    const FLAGS: [u8; 2] = 512u16.to_le_bytes();
    const OFFSET: [u8; 4] = 28u32.to_le_bytes();
    const MANIFEST_ID: [u8; 8] = 0u64.to_le_bytes();

    [
        &MAGIC[..],
        &MAJOR[..],
        &MINOR[..],
        &FLAGS[..],
        &OFFSET[..],
        &compressed_size.to_le_bytes(),
        &MANIFEST_ID[..],
        &uncompressed_size.to_le_bytes(),
    ]
    .concat()
}

fn write_file(bytes: &[u8], name: String) -> Result<(), Error> {
    let uncompressed_size = bytes.len();

    let compressed = zstd::encode_all(bytes, 19).expect("error compressing");
    let compressed_size = compressed.len();

    let head = make_header(
        compressed_size.try_into().unwrap(),
        uncompressed_size.try_into().unwrap(),
    );

    let bytes = [&head[..], &compressed[..]].concat();

    let mut file = fs::File::create(format!("./assets/{name}.manifest"))?;
    file.write_all(&bytes);

    Ok(())
}

fn valid_manifest() -> Vec<u8> {
    let mut builder = flatbuffers::FlatBufferBuilder::new();

    let chunk_0 = Chunk::create(
        &mut builder,
        &ChunkArgs {
            id: 0,
            compressed_size: 0,
            uncompressed_size: 0,
        },
    );
    let chunk_1 = Chunk::create(
        &mut builder,
        &ChunkArgs {
            id: 1,
            compressed_size: 0,
            uncompressed_size: 0,
        },
    );

    let chunks = Some(builder.create_vector(&[chunk_0, chunk_1]));

    let bundle_0 = Bundle::create(&mut builder, &BundleArgs { id: 0, chunks });

    let name = Some(builder.create_string(""));
    let dir_root = Directory::create(
        &mut builder,
        &DirectoryArgs {
            id: 0,
            parent_id: 0,
            name,
        },
    );
    let name = Some(builder.create_string("Test"));
    let dir_test = Directory::create(
        &mut builder,
        &DirectoryArgs {
            id: 1,
            parent_id: 0,
            name,
        },
    );

    let name = Some(builder.create_string("en_US"));
    let lang_en_us = Language::create(&mut builder, &LanguageArgs { id: 0, name });

    let key_0 = Key::create(&mut builder, &KeyArgs { unk0: 1, unk1: 4 });

    let param_0 = Param::create(
        &mut builder,
        &ParamArgs {
            unk0: 0,
            chunking_version: 3,
            min_chunk_size: 2,
            chunk_size: 8,
            max_chunk_size: 32,
        },
    );

    let name = Some(builder.create_string("file.txt"));
    let symlink = Some(builder.create_string(""));
    let chunk_ids = Some(builder.create_vector(&[0u64, 1]));
    let file = File::create(
        &mut builder,
        &FileArgs {
            id: 0,
            directory_id: 1,
            size_: 0,
            name,
            language_mask: 0,
            unk5: 0,
            unk6: 0,
            chunk_ids,
            unk8: 0,
            symlink,
            unk10: 0,
            param_id: 0,
            permissions: 0,
        },
    );

    let bundles = Some(builder.create_vector(&[bundle_0]));
    let languages = Some(builder.create_vector(&[lang_en_us]));
    let files = Some(builder.create_vector(&[file]));
    let directories = Some(builder.create_vector(&[dir_root, dir_test]));
    let keys = Some(builder.create_vector(&[key_0]));
    let params = Some(builder.create_vector(&[param_0]));

    let manifest = Manifest::create(
        &mut builder,
        &ManifestArgs {
            bundles,
            languages,
            files,
            directories,
            keys,
            params,
        },
    );

    builder.finish(manifest, None);

    builder.finished_data().to_vec()
}

fn valid_empty_manifest() -> Vec<u8> {
    let mut builder = flatbuffers::FlatBufferBuilder::new();

    let bundles = Some(builder.create_vector::<WIPOffset<Bundle>>(&[]));
    let languages = Some(builder.create_vector::<WIPOffset<Language>>(&[]));
    let files = Some(builder.create_vector::<WIPOffset<File>>(&[]));
    let directories = Some(builder.create_vector::<WIPOffset<Directory>>(&[]));
    let keys = Some(builder.create_vector::<WIPOffset<Key>>(&[]));
    let params = Some(builder.create_vector::<WIPOffset<Param>>(&[]));

    let manifest = Manifest::create(
        &mut builder,
        &ManifestArgs {
            bundles,
            languages,
            files,
            directories,
            keys,
            params,
        },
    );

    builder.finish(manifest, None);

    builder.finished_data().to_vec()
}

fn main() -> Result<(), Error> {
    fs::create_dir_all("./assets")?;

    write_file(&valid_manifest(), "valid".to_owned()).unwrap();
    write_file(&valid_empty_manifest(), "valid_empty".to_owned()).unwrap();

    Ok(())
}
