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
        /// Nonce has overflowed past u64 limits
        NonceOverflow,
    }

    // Events.
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // Declare events
        Created(T::AccountId, T::Hash),
        PriceSet(T::AccountId, T::Hash, T::Balance),
        Transferred(T::AccountId, T::AccountId, T::Hash),
        Bought(T::AccountId, T::AccountId, T::Hash, T::Balance),
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

    // ACTION #12: Our pallet's genesis configuration.

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        
        // create_kitty
        #[pallet::weight(100)]
        pub fn create_kitty(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            let random_hash = Self::random_hash(&sender);

            let new_kitty = Kitty {
                id: random_hash,
                dna: random_hash,
                price: 0u8.into(),
                gender: Kitty::<T, T>::gender(random_hash),
            };

            Self::mint(sender, random_hash, new_kitty)?;
            Self::increment_nonce()?;

            Ok(().into())
        }
        
        // set_price
        #[pallet::weight(100)]
        pub fn set_price(
            origin: OriginFor<T>,
            kitty_id: T::Hash,
            new_price: T::Balance,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            // Make sure the Kitty exists.
            ensure!(
                <Kitties<T>>::contains_key(kitty_id),
                "This cat does not exist"
            );
            // ACTION #1: Checking Kitty owner

            // ACTION #2: Set the Kitty price and update new Kitty infomation to storage.

            // ACTION #3: Deposit a "PriceSet" event.

            Ok(().into())
        }

        // ACTION #5: transfer

        // buy_kitty
        #[pallet::weight(100)]
        pub fn buy_kitty(
            origin: OriginFor<T>,
            kitty_id: T::Hash,
            ask_price: T::Balance,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            // Check if the Kitty exists.
            ensure!(
                <Kitties<T>>::contains_key(kitty_id),
                "This cat does not exist"
            );

            // Check that the Kitty has an owner.
            let owner = Self::owner_of(kitty_id).ok_or("No owner for this kitty")?;

            // Check that account buying the Kitty doesn't already own it.
            ensure!(owner != sender, "You can't buy your own cat");

            // Get the price of the Kitty.
            let mut kitty = Self::kitty(kitty_id);
            let kitty_price = kitty.price;

            // ACTION #5: Check if the Kitty is for sale.
            
            // ACTION #6: Update Balances using the Currency trait.

            // ACTION #7: Transfer ownership of Kitty and set new price.

            Self::deposit_event(Event::Bought(sender, owner, kitty_id, kitty_price));

            Ok(().into())
        }
        
        // breed_kitty
        #[pallet::weight(100)]
        pub fn breed_kitty(
            origin: OriginFor<T>,
            kitty_id_1: T::Hash,
            kitty_id_2: T::Hash,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            ensure!(
                <Kitties<T>>::contains_key(kitty_id_1),
                "This cat 1 does not exist"
            );
            ensure!(
                <Kitties<T>>::contains_key(kitty_id_2),
                "This cat 2 does not exist"
            );

            // ACTION #8: Breed two Kitties using unique DNA

            // ACTION #9: Mint new Kitty using new DNA and increment nonce

            Ok(().into())
        }
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
                let next = nonce.checked_add(1).ok_or(Error::<T>::NonceOverflow)?;
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

		// Helper to mint a Kitty.
        fn mint(
            to: T::AccountId,
            kitty_id: T::Hash,
            new_kitty: Kitty<T::Hash, T::Balance>,
        ) -> DispatchResult {
            ensure!(
                !<KittyOwner<T>>::contains_key(kitty_id),
                "Kitty already contains_key"
            );

            // Write mint function
            // Update total Kitty counts.
            let owned_kitty_count = Self::owned_kitty_count(&to);
            let new_owned_kitty_count = owned_kitty_count
                .checked_add(1)
                .ok_or("Overflow adding a new kitty to account balance")?;

            let all_kitties_count = Self::all_kitties_count();
            let new_all_kitties_count = all_kitties_count
                .checked_add(1)
                .ok_or("Overflow adding a new kitty to total supply")?;

            // Update storage with new Kitty.
            <Kitties<T>>::insert(kitty_id, new_kitty);
            <KittyOwner<T>>::insert(kitty_id, Some(&to));

            // Write Kitty counting information to storage.
            <AllKittiesArray<T>>::insert(new_all_kitties_count, kitty_id);
            <AllKittiesCount<T>>::put(new_all_kitties_count);
            <AllKittiesIndex<T>>::insert(kitty_id, new_all_kitties_count);

            // Write Kitty counting information to storage.
            <OwnedKittiesArray<T>>::insert((to.clone(), new_owned_kitty_count), kitty_id);
            <OwnedKittiesCount<T>>::insert(&to, new_owned_kitty_count);
            <OwnedKittiesIndex<T>>::insert(kitty_id, new_owned_kitty_count);

            // Write `mint` event
            Self::deposit_event(Event::Created(to, kitty_id));

			Ok(())
        }

		// ACTION #6: Write transfer_from
        
    }
}