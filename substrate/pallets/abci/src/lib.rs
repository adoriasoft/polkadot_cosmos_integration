#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

use frame_support::{
    debug, decl_module, decl_storage, dispatch::DispatchResult, dispatch::Vec, sp_runtime::print,
    sp_runtime::transaction_validity::TransactionSource, weights::Weight,
};
use sp_std::prelude::*;
use frame_system::{
    ensure_signed,
    offchain::{AppCrypto, CreateSignedTransaction, SendSignedTransaction, Signer},
};
use lite_json::json::JsonValue;
use sp_runtime::offchain::{http, Duration};

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
            let res = Self::make_request();
            match res {
                Ok(results) => {
                    debug::native::info!("Results: {:?}", results.len());
                    for val in &results {
                        match val {
                            Ok(acc) => debug::info!("Submitted transaction: {:?}", acc),
                            Err(e) => debug::error!("Failed to submit transaction: {:?}", e),
                        }
                    }
                }
                Err(e) => {
                    debug::error!("Error: {}", e);
                }
            }
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

    pub fn make_request() -> Result<Vec<Result<T::AccountId, ()>>, &'static str> {
        let signer = Signer::<T, T::AuthorityId>::all_accounts();
        if !signer.can_sign() {
            return Err(
                "No local accounts available. Consider adding one via `author_insertKey` RPC.",
            )?;
        }
        let requests: Vec<u32> = Self::requests();
        let mut res = vec![];
        for request_id in requests {
            debug::native::info!("Request #{:?}", request_id);
            print("Validate from pallet");
            let result = Self::fetch_price().map_err(|_| "Failed to fetch price")?;
            res.push(result);
            // res.push(request_id);
        }
        // Todo: Make gRPC request
        if res.len() > 0 {
            let results = signer.send_signed_transaction(|_account| Call::finish_deliver_tx(res.clone()));
            Ok(results
                .iter()
                .map(|(acc, res)| match res {
                    Ok(_) => Ok(acc.id.clone()),
                    Err(_) => Err(()),
                })
                .collect())
        } else {
            Ok(vec![])
        }
    }

    /// Fetch current price and return the result in cents.
    pub fn fetch_price() -> Result<u32, http::Error> {
        // We want to keep the offchain worker execution time reasonable, so we set a hard-coded
        // deadline to 2s to complete the external call.
        // You can also wait idefinitely for the response, however you may still get a timeout
        // coming from the host machine.
        let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(2_000));
        // Initiate an external HTTP GET request.
        // This is using high-level wrappers from `sp_runtime`, for the low-level calls that
        // you can find in `sp_io`. The API is trying to be similar to `reqwest`, but
        // since we are running in a custom WASM execution environment we can't simply
        // import the library here.
        let request =
            http::Request::get("https://min-api.cryptocompare.com/data/price?fsym=BTC&tsyms=USD");
        // We set the deadline for sending of the request, note that awaiting response can
        // have a separate deadline. Next we send the request, before that it's also possible
        // to alter request headers or stream body content in case of non-GET requests.
        let pending = request
            .deadline(deadline)
            .send()
            .map_err(|_| http::Error::IoError)?;

        // The request is already being processed by the host, we are free to do anything
        // else in the worker (we can send multiple concurrent requests too).
        // At some point however we probably want to check the response though,
        // so we can block current thread and wait for it to finish.
        // Note that since the request is being driven by the host, we don't have to wait
        // for the request to have it complete, we will just not read the response.
        let response = pending
            .try_wait(deadline)
            .map_err(|_| http::Error::DeadlineReached)??;
        // Let's check the status code before we proceed to reading the response.
        if response.code != 200 {
            debug::warn!("Unexpected status code: {}", response.code);
            return Err(http::Error::Unknown);
        }

        // Next we want to fully read the response body and collect it to a vector of bytes.
        // Note that the return object allows you to read the body in chunks as well
        // with a way to control the deadline.
        let body = response.body().collect::<Vec<u8>>();

        // Create a str slice from the body.
        let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
            debug::warn!("No UTF8 body");
            http::Error::Unknown
        })?;

        let price = match Self::parse_price(body_str) {
            Some(price) => Ok(price),
            None => {
                debug::warn!("Unable to extract price from the response: {:?}", body_str);
                Err(http::Error::Unknown)
            }
        }?;

        debug::warn!("Got price: {} cents", price);

        Ok(price)
    }

    /// Parse the price from the given JSON string using `lite-json`.
    ///
    /// Returns `None` when parsing failed or `Some(price in cents)` when parsing is successful.
    fn parse_price(price_str: &str) -> Option<u32> {
        let val = lite_json::parse_json(price_str);
        let price = val.ok().and_then(|v| match v {
            JsonValue::Object(obj) => {
                let mut chars = "USD".chars();
                obj.into_iter()
                    .find(|(k, _)| k.iter().all(|k| Some(*k) == chars.next()))
                    .and_then(|v| match v.1 {
                        JsonValue::Number(number) => Some(number),
                        _ => None,
                    })
            }
            _ => None,
        })?;

        let exp = price.fraction_length.checked_sub(2).unwrap_or(0);
        Some(price.integer as u32 * 100 + (price.fraction / 10_u64.pow(exp)) as u32)
    }
}
