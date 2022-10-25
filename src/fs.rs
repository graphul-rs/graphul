#[derive(Clone, Debug)]
pub struct FolderConfig {
    pub spa: bool,
    pub compress: bool,
    pub chunk_size: Option<usize>,
    pub index: bool,
    pub not_found: Option<&'static str>,
}

impl FolderConfig {
    pub fn spa() -> Self {
        Self {
            spa: true,
            compress: true,
            chunk_size: None,
            index: true,
            not_found: None,
        }
    }
}

impl Default for FolderConfig {
    fn default() -> Self {
        Self {
            spa: false,
            compress: true,
            chunk_size: None,
            index: Default::default(),
            not_found: Default::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FileConfig {
    pub compress: bool,
    pub chunk_size: Option<usize>,
}

impl Default for FileConfig {
    fn default() -> Self {
        Self {
            compress: true,
            chunk_size: None,
        }
    }
}
