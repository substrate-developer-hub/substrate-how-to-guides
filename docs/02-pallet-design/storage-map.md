---
sidebar_position: 8
keywords: pallet design, beginner, runtime
---

# Create and use a storage Map

_Learn how to write into a storage map._

## Goal

Learn how to use a [`StorageMap`][storagemap-rustdocs] using FRAME V2's syntax.

## Use cases

Declaring a Storage Map and using it in a pallet.

## Overview

We will create 2 maps :

- `WhenLastSomethingDone` : to store the lastest block number when a function is called
- `CallsCounterBySender` : to store a counter by sender address for a function called

## Steps

### 1. Define a StorageMap instance

Use FRAME's `StorageMap` to declare the struct as a new single item in storage:

```rust
#[pallet::storage]
#[pallet::getter(fn when_last_something_done)]
pub type WhenLastSomethingDone<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, T::BlockNumber, ValueQuery>;

#[pallet::storage]
#[pallet::getter(fn call_counter_by_sender)]
pub type CallsCounterBySender<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, u32, ValueQuery>;

```

### 2. Write to the `WhenLastSomethingDone` storage item

Example of using `WhenLastSomethingDone` inside a function using `insert` :

```rust
let who = ensure_signed(origin)?;
let now = frame_system::Pallet::<T>::block_number();
<WhenLastSomethingDone<T>>::insert(&who, now);
```

### 3. Write to the `CallsCounterBySender` storage item

Example of the `CallsCounterBySender` StorageMap being used inside a function:

```rust
let who = ensure_signed(origin)?;
if ! <CallsCounterBySender<T>>::contains_key(&who) {
<CallsCounterBySender<T>>::insert(&who, 0);
}
let counter = <CallsCounterBySender<T>>::get(&who).saturating_add(1);
<CallsCounterBySender<T>>::insert(&who, counter);

```

## Examples

- Inside FRAME's [`pallet-balances`](https://github.com/paritytech/substrate/blob/37d4bce3f478cab6903401a9089449a27eb24a38/frame/balances/src/lib.rs#L485-L497)
- StorageMaps in the [`archipel project`](https://github.com/luguslabs/archipel/blob/upgrade-substrate-3.0.0/chain/pallets/archipel/src/lib.rs#L39-L75)

## Resources

#### Rust docs
- [StorageMap][storagemap-rustdocs]

[storagemap-rustdocs]: https://crates.parity.io/frame_support/storage/trait.StorageMap.html
