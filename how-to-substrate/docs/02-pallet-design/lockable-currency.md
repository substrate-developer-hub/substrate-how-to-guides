---
sidebar_position: 5
keywords: pallet design, currency, intermediate, runtime engineering
---

# Basic staking pallet
_Build a composable staking pallet for your runtime._

## Goal
Write a pallet that allows users to lock funds.

## Use cases
- Staking
- Conviction voting 

## Overview
The [`LockableCurrency`][lockable-rustdocs] trait can be very useful in the context of economic systems that enforce accountability by collateralizing fungible resources.
Substrate's [staking pallet][staking-frame] makes use of the same trait to handle locked funds in time-based increments. 

## Steps

### 1. Declare relevant imports

Import the following traits:

```rust
use frame_support::{
	dispatch::DispatchResult,
	traits::{Currency, LockIdentifier, LockableCurrency, WithdrawReasons},
};
```

### 2.Declare the `LockIdentifier` constant
In order to use `LockableCurrency`, we need to declare a [`LockIdentifier`][lockidentifier-rustdocs]: 

```rust
const EXAMPLE_ID: LockIdentifier = *b"example ";
```

### 3. Define the required types

Define the lockable currency type:

```rust
type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
/*--snip--*/
	type Currency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;
```
The new `BalanceOf<T>` type satisfies the type constraints of `Self::Balance` for all provided 
methods in`Currency`. 

### 4. Write out the required functions

Our pallet must contain the following key functions:
- `fn lock_capital`: Locks the specified amount of tokens from the caller.
- `fn extend_lock`: Extends the lock period.
- `fn unlock_all`: Releases all locked tokens.

#### `fn lock_capital`
Call the `set_lock()` method from `Currency`:

```rust
/// Locks the specified amount of tokens from the caller
		#[weight = 10_000]
		fn lock_capital(origin, amount: BalanceOf<T>) -> DispatchResult {
			let user = ensure_signed(origin)?;

			T::Currency::set_lock(
				EXAMPLE_ID,
				&user,
				amount,
				WithdrawReasons::all(),
			);

			Self::deposit_event(RawEvent::Locked(user, amount));
			Ok(())
		}
```
#### `fn extend_lock`
Call the `extend_lock()` method from `Currency`:

```rust
#[weight = 10_000]
		fn extend_lock(origin, amount: BalanceOf<T>) -> DispatchResult {
			let user = ensure_signed(origin)?;

			T::Currency::extend_lock(
				EXAMPLE_ID,
				&user,
				amount,
				WithdrawReasons::all(),
			);

			Self::deposit_event(RawEvent::ExtendedLock(user, amount));
			Ok(())
		}
```

#### `fn unlock_all` 
Call the `remove_lock()` method from `Currency`:

```rust
/// Releases all locked tokens
		#[weight = 10_000]
		fn unlock_all(origin) -> DispatchResult {
			let user = ensure_signed(origin)?;

			T::Currency::remove_lock(EXAMPLE_ID, &user);

			Self::deposit_event(RawEvent::Unlocked(user));
			Ok(())
		}

```

## Examples
- [lockable-currency-pallet](../template-node/pallets/lockable-currency)

>**Further learning 💡 ** 
> - **Using `LockIdentifier`.** Use this to define logic within the runtime that schedules locking, unlocking, or extends existing locks.
> - **Composability.** Use this basic scheme when writing other pallets that borrow useful methods from mainstream traits.

## Related material
#### Rust docs
- [Currency trait](https://substrate.dev/rustdocs/v3.0.0/frame_support/traits/trait.Currency.html)

todo

[staking-frame]: https://substrate.dev/rustdocs/v3.0.0/pallet_staking/index.html
[lockable-rustdocs]: https://substrate.dev/rustdocs/v3.0.0/frame_support/traits/trait.LockableCurrency.html
[lockidentifier-rustdocs]: https://substrate.dev/rustdocs/v3.0.0/frame_support/traits/type.LockIdentifier.html