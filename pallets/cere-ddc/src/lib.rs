#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	decl_error, decl_event, decl_module, decl_storage,
	// dispatch,
	ensure,
	traits::{ Get },
};
use frame_system::ensure_signed;

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame
use sp_std::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

	/// The minimum length a name may be.
	type MinLength: Get<usize>;

	/// The maximum length a name may be.
	type MaxLength: Get<usize>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as CereDDCModule {
		/// The lookup table for string.
		StringDataOf: map hasher(twox_64_concat) T::AccountId => Option<Vec<u8>>;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Trait>::AccountId,
	{
		/// A data string was set. \[who\]
		DataStringSet(AccountId),
	
		/// A data string was changed. \[who\]
		DataStringChanged(AccountId),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// A name is too short.
		TooShort,
		/// A name is too long.
		TooLong,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		/// The minimum length a name may be.
		const MinLength: u32 = T::MinLength::get() as u32;

		/// The maximum length a name may be.
		const MaxLength: u32 = T::MaxLength::get() as u32;

		#[weight = 50_000_000]
		fn send_data(origin, send_to: T::AccountId, data: Vec<u8>) {
			let sender = ensure_signed(origin)?;

			ensure!(data.len() >= T::MinLength::get(), Error::<T>::TooShort);
			ensure!(data.len() <= T::MaxLength::get(), Error::<T>::TooLong);

			if let Some(_) = <StringDataOf<T>>::get(&sender) {
				Self::deposit_event(RawEvent::DataStringChanged(sender.clone()));
			} else {
				Self::deposit_event(RawEvent::DataStringSet(sender.clone()));
			};

			<StringDataOf<T>>::insert(send_to, data);
		}
	}
}
