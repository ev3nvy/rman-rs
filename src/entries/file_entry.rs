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
    pub symlink: String,
    pub unk10: u16,
    pub param_id: u8,
    pub permissions: u8,
}

impl From<File<'_>> for FileEntry {
    fn from(file: File) -> Self {
        let id = file.id();
        let directory_id = file.directory_id();
        let size = file.size_();
        let name = file.name().unwrap_or_default().to_owned();
        let language_mask = file.language_mask();
        let unk5 = file.unk5();
        let unk6 = file.unk6();
        let chunk_ids = file.chunk_ids().unwrap_or_default();
        let unk8 = file.unk8();
        let symlink = file.symlink().unwrap_or_default().to_owned();
        let unk10 = file.unk10();
        let param_id = file.param_id();
        let permissions = file.permissions();

        let chunk_ids = chunk_ids.iter().collect();

        Self {
            id,
            directory_id,
            size,
            name,
            language_mask,
            unk5,
            unk6,
            chunk_ids,
            unk8,
            symlink,
            unk10,
            param_id,
            permissions,
        }
    }
}
