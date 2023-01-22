use std::io::Error;

use crate::generated::rman::File;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct FileEntry {
    pub id: u64,
    pub directory_id: u64,
    pub size: u32,
    pub name: String,
    pub language_mask: u64,
    pub unk5: u8,
    pub unk6: u8,
    pub chunk_ids: Vec<u64>,
    pub unk8: u8,
    pub link: String,
    pub unk10: u16,
    pub params_index: u8,
    pub permissions: u8,
}

impl TryFrom<File<'_>> for FileEntry {
    type Error = Error;

    fn try_from(file: File) -> Result<Self, Self::Error> {
        let id = file.id();
        let directory_id = file.directory_id();
        let size = file.size_();
        let name = file.name().unwrap_or_default().to_string();
        let language_mask = file.language_mask();
        let unk5 = file.unk5();
        let unk6 = file.unk6();
        let chunk_ids = file.chunk_ids().unwrap_or_default();
        let unk8 = file.unk8();
        let link = file.link().unwrap_or_default().to_string();
        let unk10 = file.unk10();
        let params_index = file.params_index();
        let permissions = file.permissions();

        let chunk_ids = chunk_ids.iter().collect();

        Ok(Self {
            id,
            directory_id,
            size,
            name,
            language_mask,
            unk5,
            unk6,
            chunk_ids,
            unk8,
            link,
            unk10,
            params_index,
            permissions,
        })
    }
}
