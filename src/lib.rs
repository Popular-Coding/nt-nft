#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;//To Be Updated

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]

		/// Permissionless
		CreateCollection(u32, T::AccountId),
        AssignNTNFT(u32, T::AccountId),
		AcceptAssignment(u32, T::AccountId),
		CancelAssignment(u32, T::AccountId),

		/// Permissioned
		DestroyCollection(u32, T::AccountId),
		MintNTNFT(u32, T::AccountId),
		BurnNTNFT(u32, T::AccountId),
		FreezeCollection(u32, T::AccountId),
		ThawCollection(u32, T::AccountId),
		DiscardNTNFT(u32, T::AccountId),

		/// Force - Governance
		ForceCreate(u32, T::AccountId),
		ForceCollectionStatus(u32, T::AccountId),
		
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Dispatchable which creates a collection of nt-nfts
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_collection(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::CreateCollection(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// Dispatchable which assigns an ntnft to a wallet id
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn assign_ntnft(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::AssignNTNFT(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// Dispatchable which allows a signer to accept an ntnft assignment
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn accept_assignment(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::AcceptAssignment(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// Dispatchable that allows a signer to cancel an ntnft assignment
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn cancel_assignment(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::CancelAssignment(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// Dispatchable which allows a collection owner to destroy their collection
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn destroy_collection(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::DestroyCollection(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// Dispatchable which allows a collection owner to mint new ntnfts
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn mint_ntnft(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::MintNTNFT(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// Dispatchable which allows a collection owner to burn ntnfts
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn burn_ntnft(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::BurnNTNFT(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// Dispatchable which allows a collection owner to freeze a collection
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn freeze_collection(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::FreezeCollection(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// Dispatchable which allows a collection owner to thaw a collection
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn thaw_collection(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::ThawCollection(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// Dispatchable which allows an assigned ntnft holder to discard their ntnft
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn discard_ntnft(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::DiscardNTNFT(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// A dispatchable meant for governance that forces creation of a collection
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn force_create(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::ForceCreate(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// A dispatchable meant for governance that forces an update for collection settings
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn force_collection_status(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::ForceCollectionStatus(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}
}
