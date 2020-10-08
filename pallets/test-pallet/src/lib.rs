#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs
use frame_support::{decl_error, decl_event, decl_module, debug, traits::Get, decl_storage, dispatch};
use frame_system::{self as system, ensure_signed};
use frame_support::sp_runtime::{print, offchain::http};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	// Add other types and constants required to configure this pallet.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	type BlockFetchDur: Get<Self::BlockNumber>;
}

pub const FETCHED_CRYPTOS: [(&[u8], &[u8], &[u8]); 6] = [
  (b"BTC", b"coincap",
    b"https://api.coincap.io/v2/assets/bitcoin"),
  (b"BTC", b"cryptocompare",
    b"https://min-api.cryptocompare.com/data/price?fsym=BTC&tsyms=USD"),
  (b"ETH", b"coincap",
   b"https://api.coincap.io/v2/assets/ethereum"),
  (b"ETH", b"cryptocompare",
    b"https://min-api.cryptocompare.com/data/price?fsym=ETH&tsyms=USD"),
  (b"DAI", b"coincap",
    b"https://api.coincap.io/v2/assets/dai"),
  (b"DAI", b"cryptocompare",
    b"https://min-api.cryptocompare.com/data/price?fsym=DAI&tsyms=USD"),
];

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
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as system::Trait>::AccountId,
	{
		/// Just a dummy event.
		/// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		/// To emit this event, we call the deposit function, from our runtime functions
		SomethingStored(u32, AccountId),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Value was None
		NoneValue,
		/// Value reached maximum and cannot be incremented further
		StorageOverflow,
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

		/// Just a dummy entry point.
		/// function that can be called by the external world as an extrinsics call
		/// takes a parameter of the type `AccountId`, stores it, and emits an event
		fn offchain_worker(block: T::BlockNumber) {
			match Self::fetch_data() {
			  Ok(res) => debug::info!("Result: {}", core::str::from_utf8(&res).unwrap()),
			  Err(e) => debug::error!("Error fetch_data: {}", e),
			};
		}

		/// Another dummy entry point.
		/// takes no parameters, attempts to increment storage value, and possibly throws an error
		#[weight = 10_000]
		pub fn cause_error(origin) -> dispatch::DispatchResult {
			print("Execute cause_error");

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
	  let pending = http::Request::get("https://min-api.cryptocompare.com/data/price?fsym=BTC&tsyms=USD")
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
  
	  // Collect the result in the form of bytes
	  Ok(response.body().collect::<Vec<u8>>())
	}
  }
