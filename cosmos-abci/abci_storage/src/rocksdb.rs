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
    fn write(&mut self, key: Vec<u8>, value: Vec<u8>) -> crate::CustomStorageResult<()> {
        self.db.put(key, value)?;
        Ok(())
    }

    fn get(&mut self, key: Vec<u8>) -> crate::CustomStorageResult<Option<Vec<u8>>> {
        let val = self.db.get(key)?;
        Ok(val)
    }

    fn close(&mut self) -> crate::CustomStorageResult<()> {
        DB::destroy(&Options::default(), self.path.clone())?;
        Ok(())
    }
}
