use kvdb::DBTransaction;
use kvdb_rocksdb::{Database, DatabaseConfig};

pub struct AbciStorageRocksdb {
    db: Database,
}

impl AbciStorageRocksdb {
    pub fn init(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let db = Database::open(&DatabaseConfig::default(), path)?;
        Ok(AbciStorageRocksdb { db })
    }
}

impl crate::AbciStorage for AbciStorageRocksdb {
    fn write(&mut self, key: Vec<u8>, value: Vec<u8>) -> crate::CustomStorageResult<()> {
        let mut transaction = DBTransaction::new();
        transaction.put(0, &key, &value);
        self.db.write(transaction)?;
        Ok(())
    }

    fn get(&mut self, key: Vec<u8>) -> crate::CustomStorageResult<Option<Vec<u8>>> {
        let val = self.db.get(0, &key)?;
        Ok(val)
    }
}
