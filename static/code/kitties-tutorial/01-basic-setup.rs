// pallets/kitties/lib.rs

pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]

    // ACTION: Write your storage item for `AllKittiesCount` here.
    // HINT: Always write #[pallet::storage] before you 
    // declare any storage item.
    
    #[pallet::hooks]

    #[pallet::call]
    impl<T: Config> Pallet<T> {}
}