#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::AtLeast32BitUnsigned;
	use sp_runtime::traits::Saturating;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		
		// The type used to store balances.
		type Balance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Storage item for balances to accounts mapping.
	#[pallet::storage]
	#[pallet::getter(fn get_balance)]
	pub(super) type BalanceToAccount<T: Config> = StorageMap<
		_, 
		Blake2_128Concat, 
		T::AccountId, 
		T::Balance,
		ValueQuery
		>;

	/// Token mint can emit two Event types.
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New token supply was minted.
		MintedNewSupply(T::AccountId),
		/// Tokens were successfully transferred between accounts. [from, to, value]
		Transferred(T::AccountId, T::AccountId, T::Balance),
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	
	#[pallet::call]
	impl<T:Config> Pallet<T> {
		/// Issue an amount of tokens from any origin.
		/// 
		/// This would not make sense to have in practice in the current
		/// implementation. This is an educational ressource.
		/// 
		/// Parameters:
		/// - `amount`: The amount of tokens to mint.
		///
		/// Emits `MintedNewSupply` event when successful.
		///
		/// TODO: Add safety checks and set max issuance allowed.  
		/// Weight: `O(1)`	
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn mint(
			origin: OriginFor<T>,
			#[pallet::compact] amount: T::Balance
		) -> DispatchResultWithPostInfo {
			
			let sender = ensure_signed(origin)?;
		
			// Update storage.
			<BalanceToAccount<T>>::insert(&sender, amount);

			// Emit an event.
			Self::deposit_event(Event::MintedNewSupply(sender));
			
			// Return a successful DispatchResultWithPostInfo.
			Ok(().into())
		}
		
		/// Allow minting account to transfer a given balance to another account.
		///
		/// Parameters:
		/// - `to`: The account to receive the transfer.
		/// - `amount`: The amount of balance to transfer.
		///
		/// Emits `Transferred` event when successful.
		///
		/// TODO: Add checks on minimum balance required and maximum transferrable balance.  
		/// Weight: `O(1)`	
		#[pallet::weight(1_000)]
		pub fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			#[pallet::compact] amount: T::Balance,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let sender_balance = Self::get_balance(&sender);
			let receiver_balance = Self::get_balance(&to);

			// Calculate new balances.
			let update_sender = sender_balance.saturating_sub(amount);
			let update_to = receiver_balance.saturating_add(amount);

			// Update both accounts storage.
			<BalanceToAccount<T>>::insert(&sender, update_sender);
			<BalanceToAccount<T>>::insert(&to, update_to);

			// Emit event.
			Self::deposit_event(Event::Transferred(sender, to, amount));
			Ok(().into())
		}
	}
}
