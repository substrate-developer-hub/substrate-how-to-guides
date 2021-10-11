---
sidebar_position: 9
---

# Create a simple custom origin 

_Enforcing dispatch control for a FRAME pallets._

## Goal

Create a custom origin with a specified account ID and enforce it in a signed dispatchable.

## Use cases

- Implement a dispatchable that can only be called by the origin defined from another pallet.
- Create a special origin with Sudo-like access to specific dispatchables

## Overview

This guide goes over a simple way to assign an account ID to an origin. This origin could be used from 
within the pallet its defined in but also in another tightly coupled pallet.

## Steps
### 1. Add an Orgin type to your pallet's `Config` trait

This type will be used when you implement your pallet for your runtime:

```rust
pub trait Config: frame_system::Config {
    type Origin: From<Origin>;
	#[pallet::constant]
    type SpecialAccountId: Get<Self::AccountId>;
}
```

### 2. Create a custom enum of type Origin

```rust
// Our origins for this pallet.
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode)]
pub enum MyOrigins {
	SpecialOrigin,
}
```

### 3. Create a struct and implement `try_origin` on it

```rust
/// Helper that other pallets may use to check that a dispatch is from SpecialOrigin.
pub struct EnsureSpecialOrigin;
impl<
    O: Into<Result<MyOrigin, O>> + From<MyOrigin>> frame_support::traits::EnsureOrigin<O> for EnsureSpecialOrigin {
	type Success = ();
	fn try_origin(o: O) -> Result<Self::Success, O> {
		o.into().and_then(|o| match o {
			MyOrigin::SpecialOrigin => Ok(()),
		})
	}
}
```

### 4. Ensure origin in dispatchable

```rust
pub fn special_function(
    origin: OriginFor<T>, 
    something: u32
    ) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(who == T::SpecialAccountId::get(), DispatchError::BadOrigin);

			// Update storage with something.
			<Something<T>>::put(something);

			// Return a successful DispatchResult.
			Ok(())
		}
```

### 5. Use this origin in another pallet

#### i. Config trait

Your other pallet will need to specify an origin type in its configuration trait:

```rust
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    	type PalletOrigin: EnsureOrigin<Self::Origin>;
	}
```

#### ii. Inside a dispatchable 

```rust
pub fn special_function_here(origin: OriginFor<T>) -> DispatchResult {
			T::PalletOrigin::ensure_origin(origin)?;
			Self::deposit_event(Event::Success);
			Ok(())
		}
```

### 6. Define the origins inside `runtime/src/lib.rs`

Make sure you import your custom pallet if relevant:

```rust
use my_custom_pallet;
```

Implement your pallet for your runtime, including custom types. `SpecialAccountId` will use
`ord_parameter_types!` to provide a unique key for that account:

```rust
// Define the account ID to control the origin in my_custom_pallet, make sure to store your key pairs.
frame_support::ord_parameter_types! {
	// `subkey inspect //SpecialAccountId`
	pub const SpecialAccountId: AccountId = AccountId::from(hex_literal::hex!["60f28d2abe90a6bbfef78dc50bf798d52582b1da3868d8f470b22831e5edac73"]);
}

impl my_custom_pallet::Config for Runtime {
	type Event = Event;
	type SpecialAccountId = SpecialAccountId;
}
```
:::note
You will need to add the `hex_literal` dependency in `runtime/Cargo.toml` and remove it from `runtime-benchmarks`.
:::

Last, make sure you add the pallet to the `construct_runtime!` macro:

```rust
// --snip
OriginsPallet: my_custom_pallet::{Pallet, Call, Storage, Event<T>};
// --snip
```
## Examples

- Custom `RawOrigin` enum in FRAME'S [Collective pallet](https://substrate.dev/rustdocs/latest/src/pallet_collective/lib.rs.html#158)
- `EnsureMember` from FRAME's [Collective pallet](https://substrate.dev/rustdocs/latest/src/pallet_collective/lib.rs.html#878)

- Different origins in the [Membership pallet](https://github.com/paritytech/substrate/blob/master/frame/membership/src/lib.rs#L40-L53) 

## Resources

- [Subkey](https://substrate.dev/docs/en/knowledgebase/integrate/subkey)
- [EnsureOrigin](https://substrate.dev/rustdocs/latest/frame_support/pallet_prelude/trait.EnsureOrigin.html)
- [frame_support::ord_parameter_types!](https://substrate.dev/rustdocs/latest/frame_support/macro.ord_parameter_types.html)