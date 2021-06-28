#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use frame_support::traits::Randomness;
	use frame_support::dispatch::DispatchResult;
	use frame_support::sp_runtime::traits::Hash;
	use frame_support::traits::{Currency, ExistenceRequirement};
	use frame_support::sp_runtime::traits::Zero;
    use sp_core::H256;

    // Struct for holding Kitty information.
	#[derive(Clone, Encode, Decode, Default, PartialEq)]
	pub struct Kitty<Hash, Balance> {
		id: Hash,
		dna: Hash,
		price: Balance,
		gender: Gender,
	}

    // Set Gender type in Kitty struct. 
	#[derive(Encode, Decode, Debug, Clone, PartialEq)]
	pub enum Gender {
		Male,
		Female,
	}

	impl Default for Gender {
		fn default() -> Self {
			Gender::Male
		}
	}

	#[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: pallet_balances::Config + frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	
		/// The type of Random we want to specify for runtime.
		type KittyRandomness: Randomness<H256>; 
	}

	// Errors.
    #[pallet::error]
    pub enum Error<T> {
        /// Nonce has overflowed past u64 limits
        NonceOverflow,
    }

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Created(T::AccountId, T::Hash),
		PriceSet(T::AccountId, T::Hash, T::Balance),
		Transferred(T::AccountId, T::AccountId, T::Hash),
		Bought(T::AccountId, T::AccountId, T::Hash, T::Balance),
	}

	// Storage.

	// Keeps track of the Nonce used in the randomness generator.
	#[pallet::storage]
    #[pallet::getter(fn get_nonce)]
    pub(super) type Nonce<T: Config> = StorageValue<_, u64, ValueQuery>;

	// Stores a Kitty: it's unique traits and price.
	#[pallet::storage]
    #[pallet::getter(fn kitty)]
    pub(super) type Kitties<T: Config> =
        StorageMap<_, Twox64Concat, T::Hash, Kitty<T::Hash, T::Balance>, ValueQuery>;

	// Keeps track of what accounts own what Kitty.
    #[pallet::storage]
    #[pallet::getter(fn owner_of)]
    pub(super) type KittyOwner<T: Config> =
        StorageMap<_, Twox64Concat, T::Hash, Option<T::AccountId>, ValueQuery>;

	// An index to track of all Kitties.
    #[pallet::storage]
    #[pallet::getter(fn kitty_by_index)]
    pub(super) type AllKittiesArray<T: Config> =
        StorageMap<_, Twox64Concat, u64, T::Hash, ValueQuery>;

	// Stores the total amount of Kitties in existence. // <-- could we maybe not do this like this and use the array?
	#[pallet::storage]
	#[pallet::getter(fn all_kitties_count)]
	pub(super) type AllKittiesCount<T: Config> = StorageValue <
		_, 
		u64, 
		ValueQuery
	>;
	
	// Keeps track of all the Kitties.  // <-- why this in addition to AllKittiesArray ?
	#[pallet::storage]
    pub(super) type AllKittiesIndex<T: Config> = StorageMap <
		_, 
		Twox64Concat, 
		T::Hash, 
		u64, 
		ValueQuery
	>;

	// Keep track of who a Kitty is owned by.
	#[pallet::storage]
    #[pallet::getter(fn kitty_of_owner_by_index)]
    pub(super) type OwnedKittiesArray<T: Config> = StorageMap <
		_, 
		Twox64Concat, 
		(T::AccountId, u64), 
		T::Hash, 
		ValueQuery
	>;

	// Keeps track of the total amount of Kitties owned.
	#[pallet::storage]
    #[pallet::getter(fn owned_kitty_count)]
    pub(super) type OwnedKittiesCount<T: Config> = StorageMap <
		_, 
		Twox64Concat, 
		T::AccountId, 
		u64, 
		ValueQuery
	>;

	// Keeps track of all owned Kitties by index.
    #[pallet::storage]
    pub(super) type OwnedKittiesIndex<T: Config> = StorageMap <
		_, 
		Twox64Concat, 
		T::Hash, 
		u64, 
		ValueQuery
	>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.

	#[pallet::call]
    impl<T: Config> Pallet<T> {

		/// Create a new unique kitty.
		/// 
		/// Provides the new Kitty details to the 'mint()'
		/// helper function (sender, kitty hash, Kitty struct).
		///
		/// Calls mint() and increment_nonce().
		///
		/// Weight: `O(1)`	
        #[pallet::weight(100)]
        pub fn create_kitty(
			origin: OriginFor<T>) 
			-> DispatchResultWithPostInfo {

				let sender = ensure_signed(origin)?;
				let random_hash = Self::random_hash(&sender);
				
				let new_kitty = Kitty {
					id: random_hash,
					dna: random_hash,
					price: 0u8.into(),
					gender: Kitty::<T, T>::gender(random_hash),
				};

              Self::mint(sender, random_hash, new_kitty)?;
           // Self::increment_nonce()?;
			
            Ok(().into())
        }

		/// Set the price for a Kitty.
		///
		/// Updates Kitty price and updates storage.
		///
		/// Weight: `O(1)`	
		#[pallet::weight(100)]
        pub fn set_price(
            origin: OriginFor<T>,
            kitty_id: T::Hash,
            new_price: T::Balance,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

			// ACTION: Make sure the Kitty exists.

			// ACTION: Check that the Kitty has an owner (i.e. if it exists).

			// ACTION: Make sure the owner matches the corresponding owner.

            let mut kitty = Self::kitty(kitty_id);

            // ACTION: Set the new price for the kitty

            // ACTION: Update the kitty in storage

            // ACTION: Deposit a `PriceSet` event with relevant data
            //         - owner
            //         - kitty id
            //         - the new price

            Ok(().into())
        }

		/// Breed a Kitty.
		///
		/// Breed two kitties to create a new generation
		/// of Kitties.
 		///
		/// Weight: `O(1)`
		#[pallet::weight(100)]
        pub fn breed_kitty(
            origin: OriginFor<T>,
            kitty_id_1: T::Hash,
            kitty_id_2: T::Hash,
        ) -> DispatchResultWithPostInfo {
			
			// ACTION: Make sure this transaction is signed.

			// ACTION: Include usual checks for each kitty that the
			// user inputs. 

            let random_hash = Self::random_hash(&sender);
            let kitty_1 = Self::kitty(kitty_id_1);
            let kitty_2 = Self::kitty(kitty_id_2);

            let mut final_dna = kitty_1.dna;
            for (i, (dna_2_element, r)) in kitty_2
                .dna
                .as_ref()
                .iter()
                .zip(random_hash.as_ref().iter())
                .enumerate()
            {
                if r % 2 == 0 {
                    final_dna.as_mut()[i] = *dna_2_element;
                }
            }
			
			// ACTION: Create a new Kitty object with `final_dna`

			// ACTION: Mint the new Kitty.

			// ACTION: Increment the nonce.

            Ok(().into())
        }
	}

    impl<T: Config> Pallet<T> {

        // Helper to handle transferring a Kitty from one account to another.
		fn transfer_from(
            from: T::AccountId,
            to: T::AccountId,
            kitty_id: T::Hash,
        ) -> DispatchResult {

			// ACTION: Verify that the owner is the rightful owner of this Kitty.

			// ACTION: Get account address to send from and to.

			// ACTION:  Increase/decrease the amount of owned Kitties by 1 for either account.

			// Update storage items that require an updated index.
            let kitty_index = <OwnedKittiesIndex<T>>::get(kitty_id);
            if kitty_index != new_owned_kitty_count_from {
                let last_kitty_id = 
				    <OwnedKittiesArray<T>>::get((from.clone(), new_owned_kitty_count_from));
                    <OwnedKittiesArray<T>>::insert((from.clone(), kitty_index), last_kitty_id);
                    <OwnedKittiesIndex<T>>::insert(last_kitty_id, kitty_index);
            }

			// ACTION: Write new Kitty ownership to storage items.
            
            // ACTION: Deposit a Transferred event

        }



}