use std::path::PathBuf;
use serde::de::DeserializeOwned;

pub(crate) struct Database {
    basepath: PathBuf,
}

impl Database {
    pub(crate) fn new<P: Into<PathBuf>>(basepath: P) -> Database {
        Database {
            basepath: basepath.into(),
        }
    }

    pub fn load<REC: DeserializeOwned>(&self, name: &str) -> Result<Vec<REC>, csv::Error> {
        let input = std::fs::File::open(self.basepath.join(name))
            .expect(name);
        let mut rdr = csv::Reader::from_reader(input);
        rdr.deserialize().collect()
    }
}