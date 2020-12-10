mod rocksdb;

use lazy_static::lazy_static;
use owning_ref::MutexGuardRefMut;
use std::sync::Mutex;

lazy_static! {
    static ref ABCI_STORAGE_INSTANCE: Mutex<Option<AbciStorageType>> = Mutex::new(None);
}

type AbciStorageType = Box<dyn AbciStorage + Send>;
type CustomStorageResult<T> = Result<T, Box<dyn std::error::Error>>;

pub trait AbciStorage {
    fn write(&mut self, key: Vec<u8>, data: Vec<u8>) -> CustomStorageResult<()>;

    fn get(&mut self, key: Vec<u8>) -> CustomStorageResult<Option<Vec<u8>>>;

    // close db
    fn close(&mut self) -> CustomStorageResult<()>;
}

/// Method that set abci instance.
pub fn set_custom_storage_instance<'ret>(
    new_instance: AbciStorageType,
) -> Result<
    MutexGuardRefMut<'ret, Option<AbciStorageType>, AbciStorageType>,
    Box<dyn std::error::Error>,
> {
    let mut instance = ABCI_STORAGE_INSTANCE.lock()?;
    *instance = Some(new_instance);
    // Here we create a ref to the inner value of the mutex guard.
    // Unwrap should never panic as we set it previously.
    let res = MutexGuardRefMut::new(instance).map_mut(|mg| mg.as_mut().unwrap());
    Ok(res)
}

/// Method that return abci instance.
pub fn get_abci_instance<'ret>() -> Result<
    MutexGuardRefMut<'ret, Option<AbciStorageType>, AbciStorageType>,
    Box<dyn std::error::Error>,
> {
    let instance = ABCI_STORAGE_INSTANCE.lock()?;
    if instance.is_none() {
        panic!("abci storage instance has not been set, execute set_storage_instance before calling this function");
    }
    // Here we create a ref to the inner value of the mutex guard.
    // Unwrap should never panic as we set it previously.
    let res = MutexGuardRefMut::new(instance).map_mut(|mg| mg.as_mut().unwrap());
    Ok(res)
}
