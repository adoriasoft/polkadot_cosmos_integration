mod defaults;
pub mod grpc;
pub mod utils;

use protos::crypto;

pub use defaults::*;
pub use grpc::*;

use lazy_static::lazy_static;
use owning_ref::MutexGuardRefMut;
use std::sync::Mutex;

use mockall::automock;

lazy_static! {
    static ref ABCI_INTERFACE_INSTANCE: Mutex<Option<Box<dyn AbciInterface + Send>>> =
        Mutex::new(None);
}

type AbciResult<T> = Result<Box<T>, Box<dyn std::error::Error>>;

#[automock]
pub trait ResponseFlush {}

#[automock]
pub trait ResponseEcho {
    fn get_message(&self) -> String;

    fn set_message(&mut self, v: String);
}

#[automock]
pub trait ResponseCheckTx {
    fn get_code(&self) -> u32;
    fn get_data(&self) -> Vec<u8>;
    fn get_log(&self) -> String;
    fn get_info(&self) -> String;
    fn get_gas_wanted(&self) -> i64;
    fn get_gas_used(&self) -> i64;
    fn get_codespace(&self) -> String;

    fn set_code(&mut self, v: u32);
    fn set_data(&mut self, v: Vec<u8>);
    fn set_log(&mut self, v: String);
    fn set_info(&mut self, v: String);
    fn set_gas_wanted(&mut self, v: i64);
    fn set_gas_used(&mut self, v: i64);
    fn set_codespace(&mut self, v: String);
}

#[automock]
pub trait ResponseDeliverTx {
    fn get_code(&self) -> u32;
    fn get_data(&self) -> Vec<u8>;
    fn get_log(&self) -> String;
    fn get_info(&self) -> String;
    fn get_gas_wanted(&self) -> i64;
    fn get_gas_used(&self) -> i64;
    fn get_codespace(&self) -> String;

    fn set_code(&mut self, v: u32);
    fn set_data(&mut self, v: Vec<u8>);
    fn set_log(&mut self, v: String);
    fn set_info(&mut self, v: String);
    fn set_gas_wanted(&mut self, v: i64);
    fn set_gas_used(&mut self, v: i64);
    fn set_codespace(&mut self, v: String);
}

#[automock]
pub trait ResponseInitChain {}

#[automock]
pub trait ResponseSetOption {
    fn get_code(&self) -> u32;
    fn get_log(&self) -> String;
    fn get_info(&self) -> String;
}

#[automock]
pub trait ResponseBeginBlock {}

#[automock]
pub trait ResponseEndBlock {}

#[automock]
pub trait ResponseCommit {
    fn get_data(&self) -> Vec<u8>;
    fn get_retain_height(&self) -> i64;

    fn set_data(&mut self, v: Vec<u8>);
    fn set_retain_height(&mut self, v: i64);
}

#[automock]
pub trait ResponseInfo {
    fn get_version(&self) -> String;
    fn get_app_version(&self) -> u64;
    fn get_data(&self) -> String;
    fn get_last_block_height(&self) -> i64;
    fn get_last_block_app_hash(&self) -> Vec<u8>;
}

#[automock]
pub trait ResponseQuery {
    fn get_code(&self) -> u32;
    fn get_log(&self) -> String;
    fn get_info(&self) -> String;
    fn get_index(&self) -> i64;
    fn get_key(&self) -> Vec<u8>;
    fn get_value(&self) -> Vec<u8>;
    fn get_height(&self) -> i64;
    fn get_codespace(&self) -> String;
    fn get_proof(&self) -> Option<crypto::merkle::Proof>;

    fn set_code(&mut self, v: u32);
    fn set_log(&mut self, v: String);
    fn set_info(&mut self, v: String);
    fn set_index(&mut self, v: i64);
    fn set_key(&mut self, v: Vec<u8>);
    fn set_value(&mut self, v: Vec<u8>);
    fn set_height(&mut self, v: i64);
    fn set_codespace(&mut self, v: String);
}

#[automock]
pub trait AbciInterface {
    fn echo(&mut self, message: String) -> AbciResult<dyn ResponseEcho>;

    fn check_tx(&mut self, tx: Vec<u8>, r#type: i32) -> AbciResult<dyn ResponseCheckTx>;

    fn deliver_tx(&mut self, tx: Vec<u8>) -> AbciResult<dyn ResponseDeliverTx>;

    fn init_chain(
        &mut self,
        time_seconds: i64,
        time_nanos: i32,
        chain_id: &str,
        pub_key_types: Vec<String>,
        max_bytes: i64,
        max_gas: i64,
        max_age_num_blocks: i64,
        max_age_duration: u64,
        app_state_bytes: Vec<u8>,
    ) -> AbciResult<dyn ResponseInitChain>;

    fn set_option(&mut self, key: &str, value: &str) -> AbciResult<dyn ResponseSetOption>;

    fn begin_block(
        &mut self,
        height: i64,
        hash: Vec<u8>,
        last_block_id: Vec<u8>,
        proposer_address: Vec<u8>,
    ) -> AbciResult<dyn ResponseBeginBlock>;

    fn end_block(&mut self, height: i64) -> AbciResult<dyn ResponseEndBlock>;

    fn commit(&mut self) -> AbciResult<dyn ResponseCommit>;

    fn query(
        &mut self,
        path: String,
        data: Vec<u8>,
        height: i64,
        prove: bool,
    ) -> AbciResult<dyn ResponseQuery>;

    fn info(&mut self) -> AbciResult<dyn ResponseInfo>;

    fn flush(&mut self) -> AbciResult<dyn ResponseFlush>;
}

pub fn set_abci_instance<'ret>(
    new_instance: Box<dyn AbciInterface + Send>,
) -> Result<
    MutexGuardRefMut<'ret, Option<Box<dyn AbciInterface + Send>>, Box<dyn AbciInterface + Send>>,
    Box<dyn std::error::Error>,
> {
    let mut instance = ABCI_INTERFACE_INSTANCE.lock()?;
    *instance = Some(new_instance);
    // Here we create a ref to the inner value of the mutex guard.
    // Unwrap should never panic as we set it previously.
    let res = MutexGuardRefMut::new(instance).map_mut(|mg| mg.as_mut().unwrap());
    Ok(res)
}

pub fn get_abci_instance<'ret>() -> Result<
    MutexGuardRefMut<'ret, Option<Box<dyn AbciInterface + Send>>, Box<dyn AbciInterface + Send>>,
    Box<dyn std::error::Error>,
> {
    let instance = ABCI_INTERFACE_INSTANCE.lock()?;
    if instance.is_none() {
        panic!("abci instance has not been set, execute set_abci_instance before calling this function");
    }
    // Here we create a ref to the inner value of the mutex guard.
    // Unwrap should never panic as we set it previously.
    let res = MutexGuardRefMut::new(instance).map_mut(|mg| mg.as_mut().unwrap());
    Ok(res)
}
