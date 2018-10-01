use failure::Error;
use std::{fs::File, path::PathBuf};

pub type Result<T> = std::result::Result<T, Error>;

pub trait ResourceLoader<T: Sized> {
    fn load(text: String) -> Result<T>;
}

pub trait LoadableResource: Sized {
    type Loader: ResourceLoader<Self>;
}

pub trait ResourceDataLoader {
    fn load<T: LoadableResource>(&self, path: &str) -> Result<T>;
}

pub struct FileResourceDataLoader {
    root: std::path::PathBuf,
}

impl FileResourceDataLoader {
    pub fn new(path: &str) -> Self {
        FileResourceDataLoader {
            root: PathBuf::from(path),
        }
    }
}

impl ResourceDataLoader for FileResourceDataLoader {
    fn load<T: LoadableResource>(&self, path: &str) -> Result<T> {
        use std::io::Read;

        let filepath = self.root.join(path);
        let mut file = File::open(filepath.clone())?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;

        T::Loader::load(String::from(data))
    }
}
