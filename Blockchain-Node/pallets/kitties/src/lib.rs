#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*,
		traits::{Currency,Randomness,ExistenceRequirement},
		sp_runtime::traits::Hash,
		transactional};
	use frame_system::pallet_prelude::*;
	 #[cfg(feature = "std")]
    use serde::{Deserialize, Serialize};

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Currency: Currency<Self::AccountId>;
		type KittyRandomness: Randomness<Self::Hash, Self::BlockNumber>;

		#[pallet::constant]
		type MaxKittyOwned: Get<u32>;
    }

	type AccountOf<T> = <T as frame_system::Config>::AccountId;
	type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountOf<T>>>::Balance;

	#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum Gender {
		Male,
		Female,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T: Config> {
		pub dna: [u8; 16],
		pub price: Option<BalanceOf<T>>,
		pub gender: Gender,
		pub owner: T::AccountId,
	}

	#[pallet::storage]
	#[pallet::getter(fn kitty_cnt)]
	/// Keeps track of the number of Kitties in existence.
	pub(super) type KittyCnt<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub(super) type Kitties<T: Config> = StorageMap<_,Twox64Concat,T::Hash,Kitty<T>>;

	#[pallet::storage]
	#[pallet::getter(fn kitties_owned)]
	/// Keeps track of what accounts own what Kitty.
	pub(super) type KittiesOwned<T: Config> = StorageMap<_,Twox64Concat, T::AccountId, BoundedVec<T::Hash, T::MaxKittyOwned>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config>{
	/// A new Kitty was sucessfully created. \[sender, kitty_id\]
	Created(T::AccountId, T::Hash),
	PriceSet(T::AccountId, T::Hash, Option<BalanceOf<T>>),
	/// A Kitty was sucessfully transferred. \[from, to, kitty_id\]
	Transferred(T::AccountId, T::AccountId, T::Hash),
	/// A Kitty was sucessfully bought. \[buyer, seller, kitty_id, bid_price\]
	Bought(T::AccountId, T::AccountId, T::Hash, BalanceOf<T>),
	}

	#[pallet::error]
	pub enum Error<T> {
	/// Handles arithemtic overflow when incrementing the Kitty counter.
	KittyCntOverflow,
	/// An account cannot own more Kitties than `MaxKittyCount`.
	ExceedMaxKittyOwned,
	/// Buyer cannot be the owner.
	BuyerIsKittyOwner,
	/// Cannot transfer a kitty to its owner.
	TransferToSelf,
	/// Handles checking whether the Kitty exists.
	KittyNotExist,
	/// Handles checking that the Kitty is owned by the account transferring, buying or setting a price for it.
	NotKittyOwner,
	/// Ensures the Kitty is for sale.
	KittyNotForSale,
	/// Ensures that the buying price is greater than the asking price.
	KittyBidPriceTooLow,
	/// Ensures that an account has enough funds to purchase a Kitty.
	NotEnoughBalance,
	}

	// Our pallet's genesis configuration.
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub mykitties: Vec<(T::AccountId, [u8; 16], Gender)>,
	}

	// Required to implement default for GenesisConfig.
	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> GenesisConfig<T> {
			GenesisConfig { mykitties: vec![] }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			// When building a kitty from genesis config, we require the dna and gender to be supplied.
			for (acct, dna, gender) in &self.mykitties {
				let _ = <Pallet<T>>::mint(acct, Some(dna.clone()), Some(gender.clone()));
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(100)]
		pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?; // <- add this line
			let kitty_id = Self::mint(&sender, None, None)?; // <- add this line
			// Logging to the console
			log::info!("A kitty is born with ID: {:?}.", kitty_id); // <- add this line

			// ACTION #4: Deposit `Created` event
			Self::deposit_event(Event::Created(sender, kitty_id));
			Ok(())
		}

		#[pallet::weight(100)]
		pub fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			kitty_id: T::Hash
		) -> DispatchResult {
			let from = ensure_signed(origin)?;

			// Ensure the kitty exists and is called by the kitty owner
			ensure!(Self::is_kitty_owner(&kitty_id, &from)?, <Error<T>>::NotKittyOwner);

			// Verify the kitty is not transferring back to its owner.
			ensure!(from != to, <Error<T>>::TransferToSelf);

			// Verify the recipient has the capacity to receive one more kitty
			let to_owned = <KittiesOwned<T>>::get(&to);

			ensure!((to_owned.len() as u32) < T::MaxKittyOwned::get(), <Error<T>>::ExceedMaxKittyOwned);


			Self::transfer_kitty_to(&kitty_id, &to)?;

			Self::deposit_event(Event::Transferred(from, to, kitty_id));

			Ok(())
		}

		#[pallet::weight(100)]
		pub fn buy_kitty(
			origin: OriginFor<T>,
			kitty_id: T::Hash,
			bid_price: BalanceOf<T>
		) -> DispatchResult {
			let buyer = ensure_signed(origin)?;

			// Check the kitty exists and buyer is not the current kitty owner
			let kitty = Self::kitties(&kitty_id).ok_or(<Error<T>>::KittyNotExist)?;
			ensure!(kitty.owner != buyer, <Error<T>>::BuyerIsKittyOwner);

			// Check the kitty is for sale and the kitty ask price <= bid_price
			if let Some(ask_price) = kitty.price {
				ensure!(ask_price <= bid_price, <Error<T>>::KittyBidPriceTooLow);
			} else {
				Err(<Error<T>>::KittyNotForSale)?;
			}

			// Check the buyer has enough free balance
			ensure!(T::Currency::free_balance(&buyer) >= bid_price, <Error<T>>::NotEnoughBalance);

			// Verify the buyer has the capacity to receive one more kitty
			let to_owned = <KittiesOwned<T>>::get(&buyer);
			ensure!((to_owned.len() as u32) < T::MaxKittyOwned::get(), <Error<T>>::ExceedMaxKittyOwned);

			let seller = kitty.owner.clone();

			// Transfer the amount from buyer to seller
			T::Currency::transfer(&buyer, &seller, bid_price, ExistenceRequirement::KeepAlive)?;

			// Transfer the kitty from seller to buyer
			Self::transfer_kitty_to(&kitty_id, &buyer)?;

			Self::deposit_event(Event::Bought(buyer, seller, kitty_id, bid_price));

			Ok(())
		}

		#[pallet::weight(100)]
		pub fn breed_kitty(
			origin: OriginFor<T>,
			kid1: T::Hash,
			kid2: T::Hash
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// Check: Verify `sender` owns both kitties (and both kitties exist).
			ensure!(Self::is_kitty_owner(&kid1, &sender)?, <Error<T>>::NotKittyOwner);
			ensure!(Self::is_kitty_owner(&kid2, &sender)?, <Error<T>>::NotKittyOwner);

			let new_dna = Self::breed_dna(&kid1, &kid2)?;
			Self::mint(&sender, Some(new_dna), None)?;

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn gen_gender() -> Gender {
			let random = T::KittyRandomness::random(&b"gender"[..]).0;
			match random.as_ref()[0] % 2 {
			0 => Gender::Male,
			_ => Gender::Female,
			}
		}
		fn gen_dna() -> [u8; 16] {
			let payload = (
			T::KittyRandomness::random(&b"dna"[..]).0,
			<frame_system::Pallet<T>>::block_number(),
			);
			// payload.using_encoded(blake2_128)
			let hash = frame_support::Hashable::blake2_128(&payload.encode());
			hash
		}
		// Helper to mint a Kitty.
		pub fn mint(
			owner: &T::AccountId,
			dna: Option<[u8; 16]>,
			gender: Option<Gender>,
		) -> Result<T::Hash, Error<T>> {
			let kitty = Kitty::<T> {
			dna: dna.unwrap_or_else(Self::gen_dna),
			price: None,
			gender: gender.unwrap_or_else(Self::gen_gender),
			owner: owner.clone(),
			};

			// let kitty_id = T::Hashing::hash_of(&kitty);
			let kitty_id = T::Hashing::hash_of(&kitty);

			// Performs this operation first as it may fail
			let new_cnt = Self::kitty_cnt().checked_add(1)
			.ok_or(<Error<T>>::KittyCntOverflow)?;

			// Performs this operation first because as it may fail
			<KittiesOwned<T>>::try_mutate(&owner, |kitty_vec| {kitty_vec.try_push(kitty_id)})
			.map_err(|_| <Error<T>>::ExceedMaxKittyOwned)?;
			<Kitties<T>>::insert(kitty_id, kitty);
			<KittyCnt<T>>::put(new_cnt);
			Ok(kitty_id)
		}
		pub fn is_kitty_owner(kitty_id: &T::Hash, acct: &T::AccountId) -> Result<bool, Error<T>> {
			match Self::kitties(kitty_id) {
				Some(kitty) => Ok(kitty.owner == *acct),
				None => Err(<Error<T>>::KittyNotExist)
			}
		}
		pub fn transfer_kitty_to(
			kitty_id: &T::Hash,
			to: &T::AccountId,
		) -> Result<(), Error<T>> {
			let mut kitty = Self::kitties(&kitty_id).ok_or(<Error<T>>::KittyNotExist)?;

			let prev_owner = kitty.owner.clone();

			// Remove `kitty_id` from the KittyOwned vector of `prev_kitty_owner`
			<KittiesOwned<T>>::try_mutate(&prev_owner, |owned| {
				if let Some(ind) = owned.iter().position(|&id| id == *kitty_id) {
					owned.swap_remove(ind);
					return Ok(());
				}
				Err(())
			}).map_err(|_| <Error<T>>::KittyNotExist)?;

			// Update the kitty owner
			kitty.owner = to.clone();
			// Reset the ask price so the kitty is not for sale until `set_price()` is called
			// by the current owner.
			kitty.price = None;

			<Kitties<T>>::insert(kitty_id, kitty);

			<KittiesOwned<T>>::try_mutate(to, |vec| {
				vec.try_push(*kitty_id)
			}).map_err(|_| <Error<T>>::ExceedMaxKittyOwned)?;

			Ok(())
		}
		pub fn breed_dna(kid1: &T::Hash, kid2: &T::Hash) -> Result<[u8; 16], Error<T>> {
			let dna1 = Self::kitties(kid1).ok_or(<Error<T>>::KittyNotExist)?.dna;
			let dna2 = Self::kitties(kid2).ok_or(<Error<T>>::KittyNotExist)?.dna;

			let mut new_dna = Self::gen_dna();
			for i in 0..new_dna.len() {
				new_dna[i] = (new_dna[i] & dna1[i]) | (!new_dna[i] & dna2[i]);
			}
			Ok(new_dna)
		}
	}
}
