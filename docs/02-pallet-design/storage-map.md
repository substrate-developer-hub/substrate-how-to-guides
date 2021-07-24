---
sidebar_position: 8
keywords: pallet design, beginner, runtime
---

# Create and use a storage Map

_Learn how to write into a storage map._

## Goal

Usage of [`StorageMap`][storagemap-rustdocs] using Frame V2 syntax.

## Use cases

How to declare and insert into storage Map

## Overview

We will create 2 maps :

- WhenLastSomethingDone : to store the lastest block number when a function is called
- CallsCounterBySender : to store a counter by sender address for a function called

## Steps

### 1. Define StorageMap

Use `StorageMap` to declare the struct as a new single item in storage:

```rust
#[pallet::storage]
#[pallet::getter(fn when_last_something_done)]
pub type WhenLastSomethingDone<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, T::BlockNumber, ValueQuery>;

#[pallet::storage]
#[pallet::getter(fn call_counter_by_sender)]
pub type CallsCounterBySender<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, u32, ValueQuery>;

```

### 2. Insert into StorageMap WhenLastSomethingDone

Exemple of WhenLastSomethingDone StorageMap usage in a function :

```rust
let who = ensure_signed(origin)?;
let now = frame_system::Pallet::<T>::block_number();
<WhenLastSomethingDone<T>>::insert(&who, now);
```

### 3. Insert into StorageMap CallsCounterBySender

Exemple of CallsCounterBySender StorageMap usage in a function:

```rust
let who = ensure_signed(origin)?;
if ! <CallsCounterBySender<T>>::contains_key(&who) {
<CallsCounterBySender<T>>::insert(&who, 0);
}
let counter = <CallsCounterBySender<T>>::get(&who).saturating_add(1);
<CallsCounterBySender<T>>::insert(&who, counter);

```

## Examples

- [`archipel project example`](https://github.com/luguslabs/archipel/blob/upgrade-substrate-3.0.0/chain/pallets/archipel/src/lib.rs#L39-L75)

## Resources

#### Rust docs

[storagemap-rustdocs]: https://crates.parity.io/frame_support/storage/trait.StorageMap.html
