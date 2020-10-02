mod defaults;
pub mod grpc;

pub use defaults::*;
pub use grpc::*;

use lazy_static::lazy_static;
use owning_ref::MutexGuardRefMut;
use std::fmt::Debug;
use std::sync::Mutex;

lazy_static! {
    static ref ABCI_INTERFACE_INSTANCE: Mutex<Option<Box<dyn ABCIInterface + Send>>> =
        Mutex::new(None);
}

type AbciResult<T> = Result<Box<T>, Box<dyn std::error::Error>>;

pub trait ResponseEcho: Debug {
    fn get_message(&self) -> String;

    fn set_message(&mut self, v: String);
}

pub trait ResponseCheckTx: Debug {
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

pub trait ResponseDeliverTx: Debug {
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

pub trait ResponseInitChain: Debug {}

pub trait ResponseBeginBlock: Debug {}

pub trait ResponseEndBlock: Debug {}

pub trait ResponseCommit: Debug {
    fn get_data(&self) -> Vec<u8>;
    fn get_retain_height(&self) -> i64;

    fn set_data(&mut self, v: Vec<u8>);
    fn set_retain_height(&mut self, v: i64);
}

pub trait ResponseInfo: Debug {
    fn get_version(&self) -> String;
    fn get_app_version(&self) -> String;
    fn get_data(&self) -> String;
    fn get_last_block_height(&self) -> i64;
    fn get_last_block_app_hash(&self) -> Vec<u8>;
}

pub trait ResponseQuery: Debug {
    fn get_code(&self) -> u32;
    fn get_log(&self) -> String;
    fn get_info(&self) -> String;
    fn get_index(&self) -> i64;
    fn get_key(&self) -> Vec<u8>;
    fn get_value(&self) -> Vec<u8>;
    fn get_height(&self) -> i64;
    fn get_codespace(&self) -> String;

    fn set_code(&mut self, v: u32);
    fn set_log(&mut self, v: String);
    fn set_info(&mut self, v: String);
    fn set_index(&mut self, v: i64);
    fn set_key(&mut self, v: Vec<u8>);
    fn set_value(&mut self, v: Vec<u8>);
    fn set_height(&mut self, v: i64);
    fn set_codespace(&mut self, v: String);
}

pub trait ABCIInterface {
    fn echo(&mut self, message: String) -> AbciResult<dyn ResponseEcho>;

    fn check_tx(&mut self, tx: Vec<u8>, r#type: i32) -> AbciResult<dyn ResponseCheckTx>;

    fn deliver_tx(&mut self, tx: Vec<u8>) -> AbciResult<dyn ResponseDeliverTx>;

    fn init_chain(&mut self, genesis: &str) -> AbciResult<dyn ResponseInitChain>;

    fn begin_block(
        &mut self,
        height: i64,
        hash: Vec<u8>,
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
}

pub fn get_abci_instance<'ret>(
    abci_endpoint: &str,
) -> Result<
    MutexGuardRefMut<'ret, Option<Box<dyn ABCIInterface + Send>>, Box<dyn ABCIInterface + Send>>,
    Box<dyn std::error::Error>,
> {
    let mut instance = ABCI_INTERFACE_INSTANCE.lock()?;
    if instance.is_none() {
        *instance = Some(Box::new(ABCIInterface_grpc::connect(abci_endpoint)?));
    }
    // Here we create a ref to the inner value of the mutex guard.
    // Unwrap should never panic as we set it previously.
    let res = MutexGuardRefMut::new(instance).map_mut(|mg| mg.as_mut().unwrap());
    Ok(res)
}
