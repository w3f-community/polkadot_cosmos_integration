mod defaults;
pub mod grpc;
pub mod utils;

pub use defaults::*;
pub use grpc::*;

use lazy_static::lazy_static;
use owning_ref::MutexGuardRefMut;
use std::sync::Mutex;

use mockall::automock;

lazy_static! {
    static ref ABCI_INTERFACE_INSTANCE: Mutex<Option<AIType>> = Mutex::new(None);
}

type AIType = Box<dyn AbciInterface + Send>;
type AbciResult<T> = Result<Box<T>, Box<dyn std::error::Error>>;

/// Trait that specify fields of ResponseFlush.
#[automock]
pub trait ResponseFlush {}

/// Trait that specify fields of ResponseEcho.
#[automock]
pub trait ResponseEcho {
    fn get_message(&self) -> String;

    fn set_message(&mut self, v: String);
}

/// Trait that specify fields of ResponseCheckTx.
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

/// Trait that specify fields of ResponseDeliverTx.
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

/// Trait that specify fields of ResponseInitChain.
#[automock]
pub trait ResponseInitChain {
    fn get_validators(&self) -> Vec<protos::ValidatorUpdate>;
}

/// Trait that specify fields for ResponseSetOption.
#[automock]
pub trait ResponseSetOption {
    fn get_code(&self) -> u32;
    fn get_log(&self) -> String;
    fn get_info(&self) -> String;
}

/// Trait that specify fields for ResponseBeginBlock.
#[automock]
pub trait ResponseBeginBlock {}

/// Trait that specify fields for ResponseEndBlock.
#[automock]
pub trait ResponseEndBlock {
    fn get_validator_updates(&self) -> Vec<protos::ValidatorUpdate>;
    fn get_events(&self) -> Vec<protos::Event>;
    fn set_events(&mut self, events: Vec<protos::Event>);
    fn set_validator_updates(&mut self, validator_updates: Vec<protos::ValidatorUpdate>);
}

/// Trait that specify fields for ResponseCommit.
#[automock]
pub trait ResponseCommit {
    fn get_data(&self) -> Vec<u8>;
    fn get_retain_height(&self) -> i64;

    fn set_data(&mut self, v: Vec<u8>);
    fn set_retain_height(&mut self, v: i64);
}

/// Trait that specify fields for ResponseInfo.
#[automock]
pub trait ResponseInfo {
    fn get_version(&self) -> String;
    fn get_app_version(&self) -> u64;
    fn get_data(&self) -> String;
    fn get_last_block_height(&self) -> i64;
    fn get_last_block_app_hash(&self) -> Vec<u8>;
}

/// Trait that specify fields for ResponseQuery.
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
    fn get_proof(&self) -> Option<protos::crypto::merkle::Proof>;

    fn set_code(&mut self, v: u32);
    fn set_log(&mut self, v: String);
    fn set_info(&mut self, v: String);
    fn set_index(&mut self, v: i64);
    fn set_key(&mut self, v: Vec<u8>);
    fn set_value(&mut self, v: Vec<u8>);
    fn set_height(&mut self, v: i64);
    fn set_codespace(&mut self, v: String);
}

/// AbciInterface trait that define abci methods.
#[automock]
pub trait AbciInterface {
    fn echo(&mut self, message: String) -> AbciResult<dyn ResponseEcho>;

    fn check_tx(&mut self, tx: Vec<u8>) -> AbciResult<dyn ResponseCheckTx>;

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
        validators: Vec<protos::ValidatorUpdate>,
    ) -> AbciResult<dyn ResponseInitChain>;

    fn set_option(&mut self, key: &str, value: &str) -> AbciResult<dyn ResponseSetOption>;

    fn begin_block(
        &mut self,
        height: i64,
        hash: Vec<u8>,
        last_block_id: Vec<u8>,
        proposer_address: Vec<u8>,
        active_validators: Vec<protos::VoteInfo>,
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

/// Method that set abci instance.
pub fn set_abci_instance<'ret>(
    new_instance: AIType,
) -> Result<MutexGuardRefMut<'ret, Option<AIType>, AIType>, Box<dyn std::error::Error>> {
    let mut instance = ABCI_INTERFACE_INSTANCE.lock()?;
    *instance = Some(new_instance);
    // Here we create a ref to the inner value of the mutex guard.
    // Unwrap should never panic as we set it previously.
    let res = MutexGuardRefMut::new(instance).map_mut(|mg| mg.as_mut().unwrap());
    Ok(res)
}

/// Method that return abci instance.
pub fn get_abci_instance<'ret>(
) -> Result<MutexGuardRefMut<'ret, Option<AIType>, AIType>, Box<dyn std::error::Error>> {
    let instance = ABCI_INTERFACE_INSTANCE.lock()?;
    if instance.is_none() {
        // TODO return an error
        panic!("abci instance has not been set, execute set_abci_instance before calling this function");
    }
    // Here we create a ref to the inner value of the mutex guard.
    // Unwrap should never panic as we set it previously.
    let res = MutexGuardRefMut::new(instance).map_mut(|mg| mg.as_mut().unwrap());
    Ok(res)
}
