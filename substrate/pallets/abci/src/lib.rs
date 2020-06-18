#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
mod abci_grpc;

use frame_support::{
    debug, decl_module, decl_storage, dispatch::DispatchResult, dispatch::Vec, sp_runtime::print,
    sp_runtime::transaction_validity::TransactionSource, weights::Weight,
};
use sp_std::prelude::*;
use frame_system::{
    ensure_signed,
    offchain::{AppCrypto, CreateSignedTransaction, SendSignedTransaction, Signer},
};

pub mod crypto {
    use sp_core::crypto::KeyTypeId;
    use sp_runtime::{
        app_crypto::{app_crypto, sr25519},
        traits::Verify,
        MultiSignature,
    };
    pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"abci");
    app_crypto!(sr25519, KEY_TYPE);

    pub struct AuthId;
    impl frame_system::offchain::AppCrypto<<MultiSignature as Verify>::Signer, MultiSignature>
        for AuthId
    {
        type RuntimeAppPublic = Public;
        type GenericSignature = sp_core::sr25519::Signature;
        type GenericPublic = sp_core::sr25519::Public;
    }
}

/// The pallet's configuration trait.
pub trait Trait: CreateSignedTransaction<Call<Self>> {
    /// The identifier type for an offchain worker.
    type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
    /// The overarching dispatch call type.
    type Call: From<Call<Self>>;
}

decl_storage! {
    trait Store for Module<T: Trait> as AbciModule {
        Requests get(fn requests): Vec<u32>;
        Results get(fn results): Vec<u32>;
    }
}

// The pallet's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        /// Block initialization
        fn on_initialize(_now: T::BlockNumber) -> Weight {
            Self::do_initialize(_now);
            return 0;
        }

           /// Block finalization
        fn on_finalize() {
            Self::do_finalize();
        }

        fn offchain_worker(_now: T::BlockNumber) {
            debug::native::info!("Hello from offchain workers!");

            // Test calls
            abci_grpc::Echo();
            let data : Vec<&[u8]> = vec![b"{\"tx\": [104,101, 108, 108, 111, 32, 102, 114, 111, 109, 32, 99, 117, 114, 108, 33, 33, 33]}"];
            abci_grpc::DeliverTx(data);
        }

        #[weight = 0]
        pub fn deliver_tx(origin, id: u32) -> DispatchResult {
            ensure_signed(origin)?;
            debug::info!("Received deviler tx request #{}", id);
            <Requests>::mutate(|x| x.push(id));
            Ok(())
        }

        #[weight = 0]
        pub fn finish_deliver_tx(origin, results: Vec<u32>) -> DispatchResult {
            ensure_signed(origin)?;
            debug::native::info!("Finish deliver tx: {:?}", results);
            <Requests>::mutate(|x| *x = vec![]);
            <Results>::mutate(|x| x.extend(results));
            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    pub fn do_finalize() {
        print("Block is finilized");
    }

    pub fn do_initialize(_block_number: T::BlockNumber) {
        print("Block is initialized");
    }

    pub fn do_commit() {
        print("Block is commited")
    }

    pub fn do_check_tx(_source: TransactionSource, message: &u32) {
        print("Validate from pallet");
        print(message);
    }
}
