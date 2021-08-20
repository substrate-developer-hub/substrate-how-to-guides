#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{sp_runtime::traits::{Hash, Zero},
                        dispatch::{DispatchResultWithPostInfo, DispatchResult}, 
                        traits::{Currency, ExistenceRequirement, Randomness},
                        pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_core::H256;

    // Struct for holding Kitty information.
    #[derive(Clone, Encode, Decode, Default, PartialEq)]
    pub struct Kitty<Hash, Balance> {
        id: Hash,
        dna: Hash,
        price: Balance,
        gender: Gender,
    }    
    // Enum declaration for Gender.
    #[derive(Encode, Decode, Debug, Clone, PartialEq)]
    pub enum Gender {
        Male,
        Female,
    }

    // Implementation to handle Gender type in Kitty struct.
    impl Default for Gender {
        fn default() -> Self {
            Gender::Male
        }
    }

    #[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types it depends on.
    #[pallet::config]
    pub trait Config: pallet_balances::Config + frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        // Specify the type for Randomness we want to specify for runtime.
        type KittyRandomness: Randomness<H256, u32>;
    }

    // Errors.
    #[pallet::error]
    pub enum Error<T> {
        // ACTION #5a: Declare errors.
    }

    // Events.
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // ACTION #3: Declare events
    }

    #[pallet::storage]
    #[pallet::getter(fn all_kitties_count)]
    pub(super) type AllKittiesCount<T: Config> = StorageValue<_, u64, ValueQuery>;
    
    // The Nonce storage item.
    #[pallet::storage]
    #[pallet::getter(fn get_nonce)]
    pub(super) type Nonce<T: Config> = StorageValue<_, u64, ValueQuery>;

	// Stores a Kitty: it's unique traits and price.
    #[pallet::storage]
    #[pallet::getter(fn kitty)]
    pub(super) type Kitties<T: Config> =
        StorageMap<_, Twox64Concat, T::Hash, Kitty<T::Hash, T::Balance>, ValueQuery>;

    // Remaining storage items.

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
    pub(super) type AllKittiesIndex<T: Config> =
        StorageMap<_, Twox64Concat, T::Hash, u64, ValueQuery>;

    // Keep track of who a Kitty is owned by.
    #[pallet::storage]
    #[pallet::getter(fn kitty_of_owner_by_index)]
    pub(super) type OwnedKittiesArray<T: Config> =
        StorageMap<_, Twox64Concat, (T::AccountId, u64), T::Hash, ValueQuery>;

    // Keeps track of the total amount of Kitties owned.
    #[pallet::storage]
    #[pallet::getter(fn owned_kitty_count)]
    pub(super) type OwnedKittiesCount<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, u64, ValueQuery>;

    // Keeps track of all owned Kitties by index.
    #[pallet::storage]
    pub(super) type OwnedKittiesIndex<T: Config> =
        StorageMap<_, Twox64Concat, T::Hash, u64, ValueQuery>;

    // TODO Part IV: Our pallet's genesis configuration.

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        
        // ACTION #1: create_kitty
        
        // TODO Part IV: set_price
        
        // TODO Part IV: transfer

        // TODO Part IV: buy_kitty
        
        // TODO Part IV: breed_kitty
    }

    // Helper function for Kitty struct
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
        // Helper to increment nonce
        fn increment_nonce() -> DispatchResult {
            <Nonce<T>>::try_mutate(|nonce| {
                let next = nonce.checked_add(1).ok_or("Overflow")?; // ACTION #5b: Add error handling
                *nonce = next;

                Ok(().into())
            })
        }

        // Help to generate random value
        fn random_hash(sender: &T::AccountId) -> T::Hash {
            let nonce = <Nonce<T>>::get();
            let seed = T::KittyRandomness::random_seed();

            T::Hashing::hash_of(&(seed, &sender, nonce))
        }

		// ACTION #2: Write mint function

		// ACTION #4: Write `mint` event

			Ok(())
        }

		// TODO Part IV: Write transfer_from	
        
    }
}