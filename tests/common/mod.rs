use std::fs;
use std::io::{Error, Write};
use std::path::{Path, PathBuf};

mod generated {
    #![allow(clippy::all, clippy::pedantic, clippy::nursery)]
    #![allow(missing_debug_implementations, unused_imports)]
    include!(concat!(env!("OUT_DIR"), "/schema_generated.rs"));
}

use flatbuffers::WIPOffset;
use generated::rman::{Bundle, BundleArgs};
use generated::rman::{Chunk, ChunkArgs};
use generated::rman::{ChunkingParam, ChunkingParamArgs};
use generated::rman::{Directory, DirectoryArgs};
use generated::rman::{File, FileArgs};
use generated::rman::{Key, KeyArgs};
use generated::rman::{Manifest, ManifestArgs};
use generated::rman::{Tag, TagArgs};

trait MakeHeader {
    fn make_header(&self, compressed_size: u32, uncompressed_size: u32) -> Vec<u8> {
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
}

trait WriteFile: MakeHeader {
    fn name(&self) -> &str;

    fn write_manifest_file(&self, bytes: &[u8]) -> Result<(), Error> {
        let uncompressed_size = bytes.len();

        let compressed = zstd::encode_all(bytes, 19).expect("error compressing");
        let compressed_size = compressed.len();

        let head = self.make_header(
            compressed_size.try_into().unwrap(),
            uncompressed_size.try_into().unwrap(),
        );

        let bytes = [&head[..], &compressed[..]].concat();

        let mut path = Path::new(env!("OUT_DIR")).join(self.name());
        path.set_extension("manifest");
        let mut file = fs::File::create(path)?;
        file.write_all(&bytes)?;

        Ok(())
    }

    fn write_bundle_file(&self, bytes: &[u8]) -> Result<(u32, u32), Error> {
        let uncompressed_size = bytes.len();

        let compressed = zstd::encode_all(bytes, 19).expect("error compressing");
        let compressed_size = compressed.len();

        let mut path = Path::new(env!("OUT_DIR")).join(self.name());
        path.set_extension("bundle");
        let mut file = fs::File::create(path)?;
        file.write_all(&compressed)?;

        Ok((
            compressed_size.try_into().unwrap(),
            uncompressed_size.try_into().unwrap(),
        ))
    }
}

pub struct ValidManifest {
    name: String,
}

impl ValidManifest {
    pub fn new() -> Self {
        Self {
            name: "valid".to_owned(),
        }
    }

    pub fn path(&self) -> PathBuf {
        Path::new(env!("OUT_DIR")).join(self.name())
    }

    pub fn generate(&self) {
        let mut builder = flatbuffers::FlatBufferBuilder::new();

        let bundle_data = vec![b'T', b'E', b'S', b'T'];
        let (compressed_size, uncompressed_size) = self.write_bundle_file(&bundle_data).unwrap();

        let chunk = Chunk::create(
            &mut builder,
            &ChunkArgs {
                id: 0,
                compressed_size,
                uncompressed_size,
            },
        );

        let chunks = Some(builder.create_vector(&[chunk]));

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
        let tag_en_us = Tag::create(&mut builder, &TagArgs { id: 0, name });

        let key_0 = Key::create(&mut builder, &KeyArgs { unk0: 1, unk1: 4 });

        let param_0 = ChunkingParam::create(
            &mut builder,
            &ChunkingParamArgs {
                unk0: 0,
                chunking_version: 3,
                min_chunk_size: 2,
                chunk_size: 8,
                max_chunk_size: 32,
            },
        );

        let name = Some(builder.create_string("file.txt"));
        let symlink = Some(builder.create_string(""));
        let chunk_ids = Some(builder.create_vector(&[0i64]));
        let file = File::create(
            &mut builder,
            &FileArgs {
                id: 0,
                directory_id: 1,
                size_: 0,
                name,
                tag_bitmask: 0,
                unk5: 0,
                unk6: 0,
                chunk_ids,
                unk8: 0,
                symlink,
                unk10: 0,
                chunking_param_id: 0,
                permissions: 0,
            },
        );

        let bundles = Some(builder.create_vector(&[bundle_0]));
        let tags = Some(builder.create_vector(&[tag_en_us]));
        let files = Some(builder.create_vector(&[file]));
        let directories = Some(builder.create_vector(&[dir_root, dir_test]));
        let keys = Some(builder.create_vector(&[key_0]));
        let chunking_params = Some(builder.create_vector(&[param_0]));

        let manifest = Manifest::create(
            &mut builder,
            &ManifestArgs {
                bundles,
                tags,
                files,
                directories,
                keys,
                chunking_params,
            },
        );

        builder.finish(manifest, None);

        self.write_manifest_file(builder.finished_data()).unwrap();
    }
}

impl MakeHeader for ValidManifest {}

impl WriteFile for ValidManifest {
    fn name(&self) -> &str {
        &self.name
    }
}

pub struct ValidEmptyManifest {
    name: String,
}

impl ValidEmptyManifest {
    pub fn new() -> Self {
        Self {
            name: "valid_empty".to_owned(),
        }
    }

    pub fn path(&self) -> PathBuf {
        Path::new(env!("OUT_DIR")).join(self.name())
    }

    pub fn generate(&self) {
        let mut builder = flatbuffers::FlatBufferBuilder::new();

        let bundles = Some(builder.create_vector::<WIPOffset<Bundle>>(&[]));
        let tags = Some(builder.create_vector::<WIPOffset<Tag>>(&[]));
        let files = Some(builder.create_vector::<WIPOffset<File>>(&[]));
        let directories = Some(builder.create_vector::<WIPOffset<Directory>>(&[]));
        let keys = Some(builder.create_vector::<WIPOffset<Key>>(&[]));
        let chunking_params = Some(builder.create_vector::<WIPOffset<ChunkingParam>>(&[]));

        let manifest = Manifest::create(
            &mut builder,
            &ManifestArgs {
                bundles,
                tags,
                files,
                directories,
                keys,
                chunking_params,
            },
        );

        builder.finish(manifest, None);

        self.write_manifest_file(builder.finished_data()).unwrap();
    }
}

impl MakeHeader for ValidEmptyManifest {}

impl WriteFile for ValidEmptyManifest {
    fn name(&self) -> &str {
        &self.name
    }
}
