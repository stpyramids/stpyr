use failure::Error;
use std::{fs::File, path::PathBuf};

pub trait ResourceLoader<T: Sized> {
    fn load(&self, text: String) -> Result<T, Error>;
}

pub trait LoadableResource: Sized {
    type Loader: ResourceLoader<Self>;
}

pub trait ResourceDataLoader {
    fn load<T: LoadableResource>(&self, path: &str, loader: T::Loader) -> Result<T, Error>;
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
    fn load<T: LoadableResource>(&self, path: &str, loader: T::Loader) -> Result<T, Error> {
        use std::io::Read;

        let filepath = self.root.join(path);
        let mut file = File::open(filepath.clone())?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;

        loader.load(data)
    }
}
