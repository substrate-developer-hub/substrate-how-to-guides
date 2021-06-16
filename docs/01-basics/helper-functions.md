---
sidebar_position: 7
keywords: basics, runtime
---

# Using pallet helper functions

_Building up key components for adding functionality to your pallets._

## Goal

Include helper functions inside a pallet.

## Use cases

Re-use helper functions to perform common "verify" checks across multiple pallets.

## Overview

## Steps

### 1. Create your helper function

We'll showcase including the following helper to your pallet:
- `fn _adder`: Checks there is no overflow when adding two integers of type `u32`.

Add it at the end of your pallet:

```rust
impl<T: Config> Module<T> {
    fn _adder(num1: u32, num2: u32) -> Result<u32, &'static str> {
        num1.checked_add(num2).ok_or("Overflow when adding")
    }
} 
```

### 2. Use it in your pallet

Identify the places where you've needed to verify for overflow when performing an addition.
Use the helper function instead, for example:

```rust
    // Extrinsics callable from outside the runtime.
	 #[pallet::call]
    impl<T: Config> Pallet<T> {
    #[pallet::weight(1_000)]

    fn add_value(
        origin: OriginFor<T>,
        val_to_add: u32
        ) -> DispatchResultWithPostInfo {
            let _ = ensure_signed(origin)?;

            ensure!(val_to_add <= T::MaxAddend::get(), "value must be <= maximum add amount constant");

            // previous value got
           	let c_val = SingleValue::<T>::get();

            // checks for overflow when new value added
            let result = _adder(c_val, val_to_add); 

            <SingleValue<T>>::put(result);
            Self::deposit_event(Event::Added(c_val, val_to_add, result));
            Ok(().into())
        }
	}
```

## Examples

## Resources

#### Rust docs