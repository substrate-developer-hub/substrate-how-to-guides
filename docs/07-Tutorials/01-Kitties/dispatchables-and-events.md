---
sidebar_position: 4
keywords: pallet design, intermediate, runtime
---

# Part III: Dispatchables and Events
_Write a dispatchable function to create a Kitty using and emits its associated Event._

## Learning outcomes

:arrow_right: Write a dispatchable that updates storage items using a helper function.

:arrow_right: Write and use pallet Events.

## Overview
In the previous section of this tutorial, we laid down the foundations geared to manage the ownership of our Kitties &mdash; when they
come to existence! In this part of the tutorial, we'll be putting these foundations to use 
by creating a Kitty on-chain, along with some error checking. This part can be broken down in the following sub-parts:
- **writing `create_kitty`**: an extrinsic or publicly callable function allowing an account to mint a Kitty.
- **writing `mint()`**: a helper function that updates our pallet's storage items and performs error checks, called by `create_kitty`.
- **including `Events`**: using FRAME's `#[pallet::events]` macro.

At the end of this part, we'll check that everything compiles without error and call our `create_kitty` extrinsic using the PolkadotJS Apps UI.

## Steps

### 1. Public and private functions

Before we dive right in, it's important to understand the design decisions we'll make around coding the minting and ownership management
capabilities of our Kitty pallet.

As developers, we want to make sure the code we write is efficient and elegant. Oftentimes, optimizing for one optimizes for the other.
The way we're going to set up our pallet up to achieve elegance and efficiency will be to break-up the "heavy lifting" dispatchable 
functions or extrinsics into private helper functions. This improves code readability and reusability too. We'll be using this design approach in the next section but for our immediate purposes, this entails using the public `create_kitty` dispatchable to:
- check the origin is signed
- generate a random hash with the signing account 
- create a new Kitty object using the random hash
- call a private `mint()` function
- increment the nonce using `increment_nonce()` from the previous part

In turn, we'll need to create the `mint` function to:
- check that the Kitty doesn't already exist
- update storage with the new Kitty ID (for all Kitties and for the owner's account)
- update the new total Kitty count for storage and the new owner's account
- deposit an Event to signal that a Kitty has succesfully been created

### 2. `create_kitty` dispatchable 

A dispatchable in FRAME always follows the same structure. All pallet dispatchables live under the `#[pallet::call]` macro which requires declaring the dispatchables section with `    impl<T: Config> Pallet<T> {}`. Read the 
documentation on these FRAME macros to learn how they work. All we need to know here is that they're a useful feature of FRAME that minimizes the code required to write for pallets to be properly integrated in a Substrate chain's runtime.

As per the requirement for `#[pallet::call]` described in the [its documentation][frame-macros-kb], every dispatchable function must have an associated weight to it. Weights are
an important part of developing with Substrate as they provide safe-guards around the amount of computation to fit in a block at execution time. 
[Substrate's weighting system][weights-kb] forces developers to think about the computational complexity each extrinsic carries before it is called so that 
a node will account for it's worst case, avoiding a lagging the network with extrinsics that take longer than a single block time. Also, weights are intimately linked to the fee system for a signed extrinsic. Learn more about this [here][txn-fees-kb].

:::tip Your turn!
Use the template code for this section to
help you write the `create_kitty` dispatchable. 
No need to worry about the `mint` function, we'll be going over that in the next section.
:::

Assuming you've correctly declared the macros required for the dispatchable, you should be able
to prove to yourself that everything is working correctly by running:

```bash
cargo build -p pallet-kitties
```

### 3. Updating storage items using the private `mint()` function

As seen when we wrote `create_kitty` in the previous section, `mint` will be responsible for 
writing our new unique Kitty object to the various  storage items declared in part II of this workshop.

Let's recap what these are:
- `<Kitties<T>>`: Stores a Kitty's unique traits and price, by storing the Kitty object.
- `<KittyOwner<T>>`: Keeps track of what accounts own what Kitty.
- `<AllKittiesArray<T>>`: An index to track of all Kitties.
- `<AllKittiesCount<T>>`: Stores the total amount of Kitties in existence.
- `<AllKittiesIndex<T>>`: Keeps track of all the Kitties.
- `<OwnedKittiesArray<T>>`: Keep track of who a Kitty is owned by.
- `<OwnedKittiesCount<T>>`: Keeps track of the total amount of Kitties owned.
- `<OwnedKittiesIndex<T>>`: Keeps track of all owned Kitties by index.

Let's get right to it. Our `mint()` function will take the following arguments:
- `to`: of type `T::AccountId`
- `kitty_id`: of type `T::Hash`
- `new_kitty`: of type `Kitty<T::Hash, T::Balance>`

And it will return `DispatchResult`.

:::note A quick note on function return types
`DispatchResult` vs. `DispatchResultWithPostInfo` todo.
:::

:::tip Your turn! 
Write out the skeleton of the `mint()` function. 

**HINT:** it will go towards the bottom of your pallet, under `impl<T: Config> Pallet<T> {}` 
:::

Now that we have the skeleton of the `mint()` function, let's go over what we need to write inside it. 

The first thing we'll need to do is to check that the Kitty being passed in doesn't already exist. We can use the built-in `ensure!` macro that Rust provides us along with
a method provided by FRAME's `StorageMap` called [`contains_key`][contains-key-rustdocs] to do this. `contains_key` will check if a key matches an item in existing Kitty object. And 
`ensure!` will return an error if the storage map 
contains the given kitty ID. 

Effectively, this check can be written the following way:
```rust
ensure!( !<SomeStorageMapStruct<T>>::contains_key(some_key), 
"SomeStorageMapStruct already contains_key");
```

Then, we can proceed with updating our storage items with the Kitty object passed into our function call. To do this, we'll make use of
the [`insert`][insert-rustdocs] method from our StorageMap API, using the following pattern:

```rust
<SomeStorageMapStruct<T>>::insert(some_key, new_key);
```

Finally, we'll have to compute a few variables to update 
storage items that keep track of counting indices for all Kitties as well as owned Kitties.

We can use the same pattern as we did in the previous part when we created `increment_nonce`, using Rust's `checked_add` and `ok_or`:

```rust
let new_value = previous_value.checked_add(1).ok_or("Overflow error!");
```

:::tip Your turn!
Write the remaining "storage-write" operations 
for the `mint()` function.

**HINT:** There's 8 in total. And `StorageValue` has a different method than `StorageMap` to update its storage instance &mdash; have a glance at the [methods it exposes][storage-value-rustdocs] to learn more.
:::

### 4. Create and use pallet Events

In Substrate, even though a transaction may be finalized, it does not necessarily imply that the function executed by that transaction fully succeeded.

To verify this, we emit an Event at the end of the function to not only report success, but to tell the "off-chain world" that some particular state transition has happened.

FRAME helps us easily manage and declare our pallet's events using the `#[pallet::event]` macro. With FRAME macros, events are just an enum declared like this:

```rust
#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config>{
    /// A function succeeded. [time, day] 
    Success(T::Time, T::Day),
}
```
As you can see in the above snippet, we use `#[pallet::generate_deposit(pub(super) fn deposit_event)]` which allows us to deposit a 
specifc event using this pattern:

```rust
Self::deposit_event(Event::Success(var_time, var_day)); 
```
In order to use events inside our pallet, we need to add the event type to our configuration trait. Additionally, just as 
when adding any type to our pallet's `Config` trait, we need to let our runtime know about it. This pattern is the same as when
we added the `KittyRandomness` type in the previous part.

:::note Notice that each event deposit is meant to be informative which is why it carries the various types associated with it. 

Get in the habit of documenting your event declaration so that your code is easy to read. It is convention to document events as such: `/// Description. [types]`. 

Learn more about events [here][events-rustdocs].
:::

:::tip Your turn!
Set your pallet up for handling events by adding the `Event` type to both your pallet's configuration trait and runtime implementation.
Then, write an event for `mint()` with the appropriate return types.

**HINT #1:** The ubiquitous Event type is `type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;`

**HINT #2:** Once all storage updates have been made in `mint()`, we want to inform the external world which account 
has created the Kitty, as well as what that Kitty's ID is.
:::

## Next steps
- Create a dispatchable to buy a Kitty
- Create a dispatchable to transfer a Kitty
- Create a dispatchable to breed two Kitties

[frame-macros-kb]: https://substrate.dev/docs/en/knowledgebase/runtime/macros#palletcall
[txn-fees-kb]: https://substrate.dev/docs/en/knowledgebase/runtime/fees
[weights-kb]: https://substrate.dev/docs/en/knowledgebase/learn-substrate/weight
[contains-key-rustdocs]: https://substrate.dev/rustdocs/v3.0.0/frame_support/storage/trait.StorageMap.html#tymethod.contains_key
[insert-rustdocs]: https://substrate.dev/rustdocs/v3.0.0/frame_support/storage/trait.StorageMap.html#tymethod.insert
[storage-value-rustdocs]: https://substrate.dev/rustdocs/v3.0.0/frame_support/storage/types/struct.StorageValue.html#method.put
[events-rustdocs]: https://crates.parity.io/frame_support/attr.pallet.html#event-palletevent-optional