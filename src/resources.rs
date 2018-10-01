use std::{fs::File, path::PathBuf};

pub type Result<T> = std::result::Result<T, String>;

pub trait ResourceLoader<T: Sized> {
    fn load(lines: Vec<String>) -> Result<T>;
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
        let mut file = File::open(filepath.clone())
            .map_err(|e| format!("{}: {:?}", e.to_string(), filepath))?;
        let mut data = String::new();
        file.read_to_string(&mut data)
            .map_err(|e| format!("{}: {:?}", e.to_string(), path))?;
        let lines = data
            .split(char::is_whitespace)
            .map(|s| s.to_owned())
            .collect();

        T::Loader::load(lines)
    }
}
