#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use frame_support::inherent::Vec;
// use frame_support::dispatch::fmt;

#[frame_support::pallet]
pub mod pallet {
	pub use super::*;

	#[derive(TypeInfo, Default, Encode, Decode)]
	#[scale_info(skip_type_params(T))]
	pub struct Kittys<T:Config> {
		id: u32,
		dna: Vec<u8>,
		price: u32,
		gender: Gender,
		owner: T::AccountId,
	}
	pub type Id = u32;

	#[derive(TypeInfo, Encode ,Decode, Debug)]
	pub enum Gender {
		Male,
		Female,
	}

	impl Default for Gender{
		fn default()-> Self{
			Gender::Male
		}
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type KittyID<T> = StorageValue<_, Id, ValueQuery>;


	#[pallet::storage]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	// Kitty Storaga
	pub(super) type KittyList<T: Config> = StorageMap<_, Blake2_128Concat, Id, Kittys<T>, OptionQuery>;

	#[pallet::storage]
	// Kitty is owning
	pub(super) type KittyOwner<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Kittys<T>>, OptionQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		KittyStored(Vec<u8>, T::AccountId),
		SwapKittyStored(T::AccountId, u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// Error don't exits kitty.
		NotExistKitty,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_kitty(origin: OriginFor<T>, dna: Vec<u8>, price: u32 ) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Check paramater
			let gender = Self::gen_gender(dna.clone())?;

			// get Kitty ID
			let mut current_id = <KittyID<T>>::get();
			// increa id
			current_id +=1;

			let kitty = Kittys {
				id: current_id,
				dna: dna.clone(),
				price: price,
				gender: gender,
				owner: who.clone(),
			};

			// Update storage.
			
			<KittyList<T>>::insert(current_id, &kitty);
			
			KittyID::<T>::put(current_id);

			// Update Amount of kitty owning
			let kitty_owning = <KittyOwner<T>>::get(who.clone());
			let mut kitty_owning_vec = match kitty_owning{
				None => Vec::new(),
				_	 =>	<KittyOwner<T>>::get(who.clone()).unwrap(),
			};
			kitty_owning_vec.push(kitty);
			<KittyOwner<T>>::insert(who.clone(), kitty_owning_vec);

			// Emit an event.
			Self::deposit_event(Event::KittyStored(dna, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn swap_kitty(origin: OriginFor<T>, swap_kitty_id: u32, to_account: T::AccountId) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;
			
			let kitty_op = <KittyList<T>>::get(swap_kitty_id);
			ensure!(kitty_op.is_some(), Error::<T>::NotExistKitty);
			let mut kitty = kitty_op.unwrap();

			// Update storage.
			
			// Update Owner Kitty
			let old_owner = kitty.owner;
			kitty.owner = to_account.clone();
			// Update Kitty list
			<KittyList<T>>::insert(swap_kitty_id, &kitty);

			// Update Kitty_Owner storage
			// Old Owner
			let old_owner_kittys_op = <KittyOwner<T>>::get(&old_owner);
			let mut old_owner_kittys_vec = match old_owner_kittys_op{
				None => Vec::new(),
				_	 =>	old_owner_kittys_op.unwrap(),
			};
			// remove kitty in list owner
			if let Some(index) = old_owner_kittys_vec.iter().position(|value| value.id == swap_kitty_id) {
				old_owner_kittys_vec.swap_remove(index);
			}
			<KittyOwner<T>>::insert(old_owner, old_owner_kittys_vec);

			// New Owner
			let new_owner_kittys_op = <KittyOwner<T>>::get(&to_account);
			let mut new_owner_kittys_vec = match new_owner_kittys_op{
				None => Vec::new(),
				_	 =>	new_owner_kittys_op.unwrap(),
			};
			new_owner_kittys_vec.push(kitty);
			<KittyOwner<T>>::insert(to_account.clone(), new_owner_kittys_vec);

			// Emit an event.
			Self::deposit_event(Event::SwapKittyStored(who, swap_kitty_id, to_account));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

	}
}

// helper function
impl<T> Pallet<T> {
	fn gen_gender(dna: Vec<u8>) -> Result<Gender,Error<T>>{
		let mut res = Gender::Male;
		if dna.len() % 2 !=0 {
			res = Gender::Female; 
		}
		Ok(res)
	}
}
