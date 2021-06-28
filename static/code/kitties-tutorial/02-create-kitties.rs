// pallets/kitties/lib.rs

pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    // ACTION: Part II, Step 1A-B: Write a Struct to hold Kitty information.
    // ACTION: Part II, Step 1D-E: Set Gender type in Kitty struct.
    #[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]

    // Stores the total amount of Kitties in existence.
	#[pallet::storage]
	#[pallet::getter(fn all_kitties_count)]
	pub(super) type AllKittiesCount<T: Config> = StorageValue <
		_, 
		u64, 
		ValueQuery
	>;
    
    // ACTION: Part II, Step 2: write the remaining storage items. 
    // HINT: Take a look at what each one is responsible for
    // to understand which Storage types to use.

    #[pallet::hooks]

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    // ACTION: Part II, Step 1F: write a function to configure the Kitty struct.
}