#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

// use sp_runtime::{
// 	traits::{Saturating, StaticLookup, Zero},
// 	ArithmeticError, RuntimeDebug,
// };

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct CollectionDetails<AccountId> {
		pub(super) owner: AccountId,
		pub(super) amount: u32,
		pub(super) is_frozen: bool,
	}

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo, MaxEncodedLen)]
	pub struct ItemDetails<AccountId> {
		// maybe differentiate minter from owner
		// On assignment but not yet accepted, who is the owner?
		pub(super) owner: AccountId,
		pub(super) is_assigned: bool,
		pub(super) is_accepted: bool,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type CollectionId: Member + Parameter + MaxEncodedLen + Copy;
		type ItemId: Member + Parameter + MaxEncodedLen + Copy;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn collection)]
	pub(super) type Collection<T: Config> = StorageMap<_, Blake2_128Concat, T::CollectionId, CollectionDetails<T::AccountId>, OptionQuery>;
	
	#[pallet::storage]
	#[pallet::getter(fn assignment)]
	pub(super) type Assignment<T: Config> = StorageDoubleMap<_, Blake2_128Concat, T::CollectionId, Blake2_128Concat, T::AccountId, T::ItemId, OptionQuery>;
	
	#[pallet::storage]
	#[pallet::getter(fn proposed_assignment)]
	pub(super) type ProposedAssignment<T: Config> = StorageDoubleMap<_, Blake2_128Concat, T::CollectionId, Blake2_128Concat, T::AccountId, T::ItemId, OptionQuery>;
	
	#[pallet::storage]
	#[pallet::getter(fn canceled_assignment)]
	pub(super) type CanceledAssignment<T: Config> = StorageDoubleMap<_, Blake2_128Concat, T::CollectionId, Blake2_128Concat, T::AccountId, T::ItemId, OptionQuery>;
	
	#[pallet::storage]
	#[pallet::getter(fn item)]
	pub(super) type Item<T: Config> = StorageDoubleMap<_, Blake2_128Concat, T::CollectionId, Blake2_128Concat, T::ItemId, ItemDetails<T::AccountId>, OptionQuery>;
	

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]

		/// Permissionless
		CreateCollection(T::CollectionId, T::AccountId),
        AssignNTNFT(T::AccountId, T::CollectionId, T::ItemId, T::AccountId),
		AcceptAssignment(T::CollectionId, T::ItemId, T::AccountId),
		CancelAssignment(T::AccountId, T::CollectionId, T::ItemId, T::AccountId),

		/// Permissioned
		DestroyCollection(T::CollectionId, T::AccountId),
		MintNTNFT(T::CollectionId, T::ItemId, T::AccountId),
		BurnNTNFT(T::CollectionId, T::ItemId, T::AccountId),
		FreezeCollection(T::CollectionId, T::AccountId),
		ThawCollection(T::CollectionId, T::AccountId),
		DiscardNTNFT(T::CollectionId, T::ItemId, T::AccountId),

		/// Force - Governance
		ForceCreate(T::CollectionId, T::AccountId),
		ForceCollectionStatus(T::CollectionId, T::AccountId),
		
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Dispatchable which creates a collection of nt-nfts
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_collection(origin: OriginFor<T>, collection_id: T::CollectionId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(!<Collection<T>>::contains_key(&collection_id), <Error<T>>::NoneValue);
			<Collection<T>>::insert(
				collection_id, 
				CollectionDetails {
					owner: who.clone(),
					amount: 0,
					is_frozen: false,
				});

			Self::deposit_event(Event::CreateCollection(collection_id, who));
			Ok(())
		}

		/// Dispatchable which assigns an ntnft to a wallet id
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn assign_ntnft(origin: OriginFor<T>, collection_id: T::CollectionId, ntnft_id: T::ItemId, target_address: T::AccountId) -> DispatchResult {
			// TODO:
			// - Update error handling
			// - Maybe differentiate owner from minter?
			let who = ensure_signed(origin)?;
			<Item<T>>::try_mutate(
				&collection_id, 
				&ntnft_id, 
				| maybe_item_details | -> DispatchResult {
					let item_details =
						maybe_item_details.as_mut().ok_or(<Error<T>>::NoneValue)?;
					ensure!(!item_details.is_accepted && !item_details.is_assigned, <Error<T>>::NoneValue);
					item_details.is_assigned = true;
					<ProposedAssignment<T>>::insert(&collection_id, &target_address, ntnft_id);
					Ok(())
				}
			)?;

			Self::deposit_event(Event::AssignNTNFT(who, collection_id, ntnft_id, target_address));
			Ok(())
		}

		/// Dispatchable which allows a signer to accept an ntnft assignment
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn accept_assignment(origin: OriginFor<T>, collection_id: T::CollectionId, ntnft_id: T::ItemId) -> DispatchResult {
			// TODO:
			// - assert item is not accepted
			// - assert item owner is target address
			let who = ensure_signed(origin)?;
			ensure!(<ProposedAssignment<T>>::contains_key(&collection_id, &who), <Error<T>>::NoneValue);

			<Item<T>>::try_mutate(
				&collection_id, 
				&ntnft_id, 
				| maybe_item_details | -> DispatchResult {
					let item_details =
						maybe_item_details.as_mut().ok_or(<Error<T>>::NoneValue)?;
					ensure!(
						!item_details.is_accepted && 
						item_details.is_assigned, 
						<Error<T>>::NoneValue
					);
					item_details.is_accepted = true;
					<ProposedAssignment<T>>::remove(&collection_id, &who);
					<Assignment<T>>::insert(&collection_id, &who, ntnft_id);
					Ok(())
				}
			)?;

			Self::deposit_event(Event::AcceptAssignment(collection_id, ntnft_id, who));
			Ok(())
		}

		/// Dispatchable that allows a signer to cancel an ntnft assignment
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn cancel_assignment(origin: OriginFor<T>, collection_id: T::CollectionId, ntnft_id: T::ItemId, target_address: T::AccountId) -> DispatchResult {
			// TODO:
			// - update error handling
			let who = ensure_signed(origin)?;
			ensure!(<ProposedAssignment<T>>::contains_key(&collection_id, &who), <Error<T>>::NoneValue);

			<Item<T>>::try_mutate(
				&collection_id, 
				&ntnft_id, 
				| maybe_item_details | -> DispatchResult {
					let item_details =
						maybe_item_details.as_mut().ok_or(<Error<T>>::NoneValue)?;
					ensure!(
						!item_details.is_accepted && 
						item_details.is_assigned,   
						<Error<T>>::NoneValue
					);
					item_details.is_accepted = false;
					item_details.is_assigned = false;
					<ProposedAssignment<T>>::remove(&collection_id, &who);
					<CanceledAssignment<T>>::insert(&collection_id, &target_address, ntnft_id);
					Ok(())
				}
			)?;

			
			Self::deposit_event(Event::CancelAssignment(who, collection_id, ntnft_id, target_address));
			Ok(())
		}

		/// Dispatchable which allows a collection owner to destroy their collection
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn destroy_collection(origin: OriginFor<T>, collection_id: T::CollectionId) -> DispatchResult {
			// TODO: 
			// - Permission checks
			ensure!(<Collection<T>>::contains_key(&collection_id), <Error<T>>::NoneValue);
			let who = ensure_signed(origin)?;
			for (_item, details) in <Item<T>>::drain_prefix(&collection_id) {
				let owner = details.owner;
				if details.is_assigned && details.is_accepted {
					<Assignment<T>>::remove(&collection_id, &owner);
				} else if details.is_assigned && !details.is_accepted {
					<ProposedAssignment<T>>::remove(&collection_id, &owner);
				}
				// Delete item?
			}
			<Collection<T>>::remove(&collection_id);
			Self::deposit_event(Event::DestroyCollection(collection_id, who));
			Ok(())
		}

		/// Dispatchable which allows a collection owner to mint new ntnfts
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn mint_ntnft(origin: OriginFor<T>, collection_id: T::CollectionId, ntnft_id: T::ItemId) -> DispatchResult {
			// TODO: 
			// - update the error handling
			// - make sure new value ! > max amount
			// - Make sure item doesn't already exist
			// - Make sure item not assigned, and not accepted
			let who = ensure_signed(origin)?;
			let collection_details = <Collection<T>>::get(&collection_id).ok_or(<Error<T>>::NoneValue)?;
			ensure!(!collection_details.is_frozen, <Error<T>>::NoneValue);
			<Collection<T>>::try_mutate(
				&collection_id, 
				| maybe_collection_details | -> DispatchResult {
					let collection_details =
						maybe_collection_details.as_mut().ok_or(<Error<T>>::NoneValue)?;
					let new_amount = 
						collection_details.amount.checked_add(1).ok_or(<Error<T>>::StorageOverflow)?;
					collection_details.amount = new_amount;
					let item = ItemDetails{
						owner: who.clone(),
						is_assigned: false,
						is_accepted: false,
					};
					<Item::<T>>::insert(&collection_id, &ntnft_id, item);
					Ok(())
				}
			)?;
			
			Self::deposit_event(Event::MintNTNFT(collection_id, ntnft_id, who));
			Ok(())
		}

		/// Dispatchable which allows a collection owner to burn ntnfts
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn burn_ntnft(origin: OriginFor<T>, collection_id: T::CollectionId, ntnft_id: T::ItemId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// TODO: 
			// - permissions checks
			// - problem with saturating dec gives potential underflow bug
			let collection_details = <Collection<T>>::get(&collection_id).ok_or(<Error<T>>::NoneValue)?; 
			ensure!(!collection_details.is_frozen, <Error<T>>::NoneValue);
			
			<Collection<T>>::try_mutate(
				&collection_id, 
				| maybe_collection_details | -> DispatchResult {
					let collection_details =
						maybe_collection_details.as_mut().ok_or(<Error<T>>::NoneValue)?;
					// Figure out why this isn't working:
					// For some reason importing saturating isn't bringing it into this scope
					// collection_details.amount.saturating_dec();
					let new_amount = 
						collection_details.amount.checked_sub(1).ok_or(<Error<T>>::StorageOverflow)?;
					collection_details.amount = new_amount;
					
					Ok(())
				}
			)?;
			let item = <Item<T>>::get(&collection_id, &ntnft_id).ok_or(<Error<T>>::NoneValue)?;
			let item_owner = item.owner;
			if item.is_accepted {
				<Assignment<T>>::remove(&collection_id, &item_owner);
			} else if item.is_assigned {
				<ProposedAssignment<T>>::remove(&collection_id, &item_owner);
			}
			<Item<T>>::remove(&collection_id, &ntnft_id);
			
			Self::deposit_event(Event::BurnNTNFT(collection_id, ntnft_id, who));
			Ok(())
		}

		/// Dispatchable which allows a collection owner to freeze a collection
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn freeze_collection(origin: OriginFor<T>, collection_id: T::CollectionId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// TODO:
			// - permission checks
			<Collection<T>>::try_mutate(
				&collection_id, 
				| maybe_collection_details | -> DispatchResult {
					let collection_details =
						maybe_collection_details.as_mut().ok_or(<Error<T>>::NoneValue)?;
					
					collection_details.is_frozen = true;
					Ok(())
				}
			)?;
			Self::deposit_event(Event::FreezeCollection(collection_id, who));
			Ok(())
		}

		/// Dispatchable which allows a collection owner to thaw a collection
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn thaw_collection(origin: OriginFor<T>, collection_id: T::CollectionId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			<Collection<T>>::try_mutate(
				&collection_id, 
				| maybe_collection_details | -> DispatchResult {
					let collection_details =
						maybe_collection_details.as_mut().ok_or(<Error<T>>::NoneValue)?;
					
					collection_details.is_frozen = false;
					Ok(())
				}
			)?;
			Self::deposit_event(Event::ThawCollection(collection_id, who));
			Ok(())
		}

		/// Dispatchable which allows an assigned ntnft holder to discard their ntnft
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn discard_ntnft(origin: OriginFor<T>, collection_id: T::CollectionId, ntnft_id: T::ItemId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::deposit_event(Event::DiscardNTNFT(collection_id, ntnft_id, who));
			Ok(())
		}

		/// A dispatchable meant for governance that forces creation of a collection
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn force_create(origin: OriginFor<T>, collection_id: T::CollectionId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::deposit_event(Event::ForceCreate(collection_id, who));
			Ok(())
		}

		/// A dispatchable meant for governance that forces an update for collection settings
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn force_collection_status(origin: OriginFor<T>, collection_id: T::CollectionId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::deposit_event(Event::ForceCollectionStatus(collection_id, who));
			Ok(())
		}
	}
}