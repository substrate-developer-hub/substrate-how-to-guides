// pallets/kitties/lib.rs
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use frame_support::traits::Randomness;
    use frame_support::sp_runtime::traits::Hash;
    use sp_core::H256;
    use frame_support::dispatch::DispatchResult;

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

    #[pallet::config]
    pub trait Config: pallet_balances::Config + frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The type of Random we want to specify for runtime.
        type KittyRandomness: Randomness<H256>;
    }

    // Events.

    #[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Created(T::AccountId, T::Hash),
		PriceSet(T::AccountId, T::Hash, T::Balance),
		Transferred(T::AccountId, T::AccountId, T::Hash),
		Bought(T::AccountId, T::AccountId, T::Hash, T::Balance),
	}
    
    // Storage items.

    // Stores the total amount of Kitties in existence.
    #[pallet::storage]
    #[pallet::getter(fn all_kitties_count)]
    pub(super) type AllKittiesCount<T: Config> = StorageValue <
        _, 
        u64, 
        ValueQuery
    >;   

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
	
	// Keeps track of all the Kitties.  
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

    // Errors.
    #[pallet::error]
    pub enum Error<T> {
        /// Nonce has overflowed past u64 limits
        NonceOverflow,
    }
    
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

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

                // ACTION: write a variable called random_hash 
                // by calling our private function using `sender`.
				let random_hash = Self::random_hash(&sender);
				
                // ACTION: create a new Kitty object using the random_hash
                // value from above.

                // Self::mint(sender, random_hash, new_kitty)?;
                
                // ACTION: call `increment_nonce`
			
            Ok(().into())
        }
    }

    // Helper function to configure the Kitty struct.
    impl<T: Config> Kitty<T, T> {
		pub fn gender(dna: T::Hash) -> Gender {
			if dna.as_ref()[0] % 2 == 0 {
				Gender::Male
			} else {
				Gender::Female
			}
		}
	}

    impl<T: Config> Pallet<T> {
        
		/// Safely increment the nonce, with error on overflow
        fn increment_nonce() -> DispatchResult {
            <Nonce<T>>::try_mutate(|nonce| {
                let next = nonce.checked_add(1).ok_or(Error::<T>::NonceOverflow)?;
                *nonce = next;

                Ok(().into())
            })
        }

       	/// Generate a random hash, using the nonce as part of the hash
        fn random_hash(sender: &T::AccountId) -> T::Hash {
            let nonce = <Nonce<T>>::get();
            let seed = T::KittyRandomness::random_seed();

            T::Hashing::hash_of(&(seed, &sender, nonce))
        }

    }
}
