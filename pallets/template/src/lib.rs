#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, debug,
	dispatch::DispatchResult,
};

use frame_system::{self as system, ensure_signed,
	ensure_none,
	offchain::{
		AppCrypto, CreateSignedTransaction, SendSignedTransaction,
		Signer,
	},
};
use frame_support::sp_runtime::{offchain::http};
use frame_support::traits::Vec;
use core::{convert::TryInto};
use sp_std::{
	prelude::*, str,
	collections::vec_deque::VecDeque,
};
use sp_core::crypto::KeyTypeId;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"demo");
pub const NUM_VEC_LEN: usize = 10;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Based on the above `KeyTypeId` we need to generate a pallet-specific crypto type wrapper.
/// We can utilize the supported crypto kinds (`sr25519`, `ed25519` and `ecdsa`) and augment
/// them with the pallet-specific identifier.
pub mod crypto {
	use crate::KEY_TYPE;
	use sp_core::sr25519::Signature as Sr25519Signature;
	use sp_runtime::app_crypto::{app_crypto, sr25519};
	use sp_runtime::{
		traits::Verify,
		MultiSignature, MultiSigner,
	};

	app_crypto!(sr25519, KEY_TYPE);

	pub struct TestAuthId;
	// implemented for ocw-runtime
	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}

	// implemented for mock runtime in test
	impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
		for TestAuthId
	{
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}
}

/// This is the pallet's configuration trait
pub trait Trait: system::Trait + CreateSignedTransaction<Call<Self>> {
	/// The identifier type for an offchain worker.
	type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
	/// The overarching dispatch call type.
	type Call: From<Call<Self>>;
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This pallet's storage items.
decl_storage! {
	// It is important to update your storage name so that your pallet's
	// storage items are isolated from other pallets.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
		// Just a dummy storage item.
		// Here we are declaring a StorageValue, `Something` as a Option<u32>
		// `get(fn something)` is the default getter which returns either the stored `u32` or `None` if nothing stored
		Something get(fn something): Option<u32>;

		/// A vector of recently submitted numbers. Bounded by NUM_VEC_LEN
		Numbers get(fn numbers): VecDeque<u64>;
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		/// Just a dummy event.
		/// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		/// To emit this event, we call the deposit function, from our runtime functions
		SomethingStored(u32, AccountId),
		/// Event generated when a new number is accepted to contribute to the average.
		NewNumber(Option<AccountId>, u64),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Value was None
		NoneValue,
		/// Value reached maximum and cannot be incremented further
		StorageOverflow,
		// Error returned when making signed transactions in off-chain worker
		NoLocalAcctForSigning,
		OffchainSignedTxError,
		// Error returned when making unsigned transactions in off-chain worker
		OffchainUnsignedTxError,
	}
}

// The pallet's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing errors
		// this includes information about your errors in the node's metadata.
		// it is needed only if you are using errors in your pallet
		type Error = Error<T>;

		// Initializing events
		// this is needed only if you are using events in your pallet
		fn deposit_event() = default;

		#[weight = 10000]
		pub fn submit_number_signed(origin, number: u64) -> DispatchResult {
			let who = ensure_signed(origin)?;
			debug::info!("submit_number_signed: ({}, {:?})", number, who);
			Self::append_or_replace_number(number);

			Self::deposit_event(RawEvent::NewNumber(Some(who), number));
			Ok(())
		}

		#[weight = 10000]
		pub fn submit_number_unsigned(origin, number: u64) -> DispatchResult {
			let _ = ensure_none(origin)?;
			debug::info!("submit_number_unsigned: {}", number);
			Self::append_or_replace_number(number);

			Self::deposit_event(RawEvent::NewNumber(None, number));
			Ok(())
		}

		/// Just a dummy entry point.
		/// function that can be called by the external world as an extrinsics call
		/// takes a parameter of the type `AccountId`, stores it, and emits an event
		#[weight = 10_000]
		pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
			// Check it was signed and get the signer. See also: ensure_root and ensure_none
			let who = ensure_signed(origin)?;

			// Code to execute when something calls this.
			// For example: the following line stores the passed in u32 in the storage
			Something::put(something);

			// Here we are raising the Something event
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			Ok(())
		}

	    fn offchain_worker(block: T::BlockNumber) {
			debug::info!("-------------------- offchain_worker start");

			match Self::fetch_data() {
				Ok(res) => debug::info!("Result: {}", core::str::from_utf8(&res).unwrap()),
				Err(e) => debug::error!("Error fetch_data: {}", e),
			  };
			
			// NOW I am do this, error here: i am fixing compile
			// Self::offchain_unsigned_tx(block);
			Self::offchain_signed_tx(block);

			// T::SubmitSignedTransaction::submit_signed(call);
		}
		/// Another dummy entry point.
		/// takes no parameters, attempts to increment storage value, and possibly throws an error
		#[weight = 10_000]
		pub fn cause_error(origin) -> dispatch::DispatchResult {
			// Check it was signed and get the signer. See also: ensure_root and ensure_none
			let _who = ensure_signed(origin)?;

			match Something::get() {
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					Something::put(new);
					Ok(())
				},
			}
		}
	}
}

impl<T: Trait> Module<T> {
	fn fetch_data() -> Result<Vec<u8>, &'static str> {
  
	  // Specifying the request
	  let pending = http::Request::get(	"https://horizon-testnet.stellar.org/accounts/GCBNOMIUIOJAA7GNRJXFUKZQ2B4ASX56GHNG2MY4YXZTGAUBYBQETIZV/transactions")
		.send()
		.map_err(|_| "Error in sending http GET request")?;
  
	  // Waiting for the response
	  let response = pending.wait()
		.map_err(|_| "Error in waiting http response back")?;
  
	  // Check if the HTTP response is okay
	  if response.code != 200 {
		debug::warn!("Unexpected status code: {}", response.code);
		return Err("Non-200 status code returned from http request");
	  }
  
	  let return_val = response.body().collect::<Vec<u8>>();
	  // Collect the result in the form of bytes
	  Ok(return_val)
	}

	/// Append a new number to the tail of the list, removing an element from the head if reaching
	///   the bounded length.
	fn append_or_replace_number(number: u64) {
		Numbers::mutate(|numbers| {
			if numbers.len() == NUM_VEC_LEN {
				let _ = numbers.pop_front();
			}
			numbers.push_back(number);
			debug::info!("Number vector: {:?}", numbers);
		});
	}

	// Commented out unused function
	// fn offchain_unsigned_tx(block_number: T::BlockNumber) -> Result<(), Error<T>> {
	// 	let number: u64 = block_number.try_into().unwrap_or(0) as u64;
	// 	let call = Call::submit_number_unsigned(number);

	// 	// `submit_unsigned_transaction` returns a type of `Result<(), ()>`
	// 	//   ref: https://substrate.dev/rustdocs/v2.0.0/frame_system/offchain/struct.SubmitTransaction.html#method.submit_unsigned_transaction
	// 	SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into())
	// 		.map_err(|_| {
	// 			debug::error!("Failed in offchain_unsigned_tx");
	// 			<Error<T>>::OffchainUnsignedTxError
	// 		})
	// }

	fn offchain_signed_tx(block_number: T::BlockNumber) -> Result<(), Error<T>> {
		// We retrieve a signer and check if it is valid.
		//   Since this pallet only has one key in the keystore. We use `any_account()1 to
		//   retrieve it. If there are multiple keys and we want to pinpoint it, `with_filter()` can be chained,
		//   ref: https://substrate.dev/rustdocs/v2.0.0/frame_system/offchain/struct.Signer.html
		let signer = Signer::<T, T::AuthorityId>::any_account();

		// Translating the current block number to number and submit it on-chain
		let number: u64 = block_number.try_into().unwrap_or(0) as u64;

		// `result` is in the type of `Option<(Account<T>, Result<(), ()>)>`. It is:
		//   - `None`: no account is available for sending transaction
		//   - `Some((account, Ok(())))`: transaction is successfully sent
		//   - `Some((account, Err(())))`: error occured when sending the transaction
		let result = signer.send_signed_transaction(|_acct|
			// This is the on-chain function
			Call::submit_number_signed(number)
		);

		// Display error if the signed tx fails.
		if let Some((acc, res)) = result {
			if res.is_err() {
				debug::error!("failure: offchain_signed_tx: tx sent: {:?}", acc.id);
				return Err(<Error<T>>::OffchainSignedTxError);
			}
			// Transaction is sent successfully
			return Ok(());
		}

		// The case of `None`: no account is available for sending
		debug::error!("No local account available");
		Err(<Error<T>>::NoLocalAcctForSigning)
	}
  }