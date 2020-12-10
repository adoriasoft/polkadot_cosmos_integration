use rocksdb::{Options, DB};

pub struct AbciStorageRocksdb {
    db: DB,
    path: String,
}

impl AbciStorageRocksdb {
    pub fn init(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let db = DB::open_default(path)?;
        Ok(AbciStorageRocksdb {
            db: db,
            path: path.to_string(),
        })
    }
}

impl crate::AbciStorage for AbciStorageRocksdb {
    fn write(&mut self, key: Vec<u8>, value: Vec<u8>) -> CustomStorageResult<()> {
        self.db.put(key, value)?;
    }

    fn get(&mut self, key: Vec<u8>) -> CustomStorageResult<Vec<u8>> {
        self.db.get(key)?
    }

    fn close(&mut self) -> CustomStorageResult<()> {
        DB::destroy(&Options::default(), self.path);
    }
}
