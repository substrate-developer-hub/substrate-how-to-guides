---
sidebar_position: 3
keywords: pallet design, intermediate, runtime
---

# Uniqueness, custom types and storage maps

_In this part of the tutorial, we'll build out the components of our pallet
needed to manage the creation and ownership of our Kitties._

## Overview

This part of the tutorial dives into some pillar concepts for developing pallets with FRAME. Ontop of learning
how to use existing types and traits, you'll learn how create your own types like providing your pallet with
a Gender type.
At the end of this part, you will have implemented the 2 remaining storage items according to the logic outlined for the Substrate Kitty 
application [in the overview of this tutorial](../overview#what-were-building).

## Learning outcomes

:arrow_right: Writing a custom struct and using it in storage.

:arrow_right: Creating a custom type and implementing it for your pallet's `Config` trait.

:arrow_right: Using the Randomness trait in a helper function.

:arrow_right: Adding `StorageMap` items to a pallet.

## Steps

### 1. Kitty struct scaffolding

We added additional comments to the code from Part I (in the `/pallets/mykitties/src/lib.rs` file) to better assist you with the action items in this part of the tutorial. To follow each step with ease, 
you can just replace your code with the [helper code][helper-code-pt2] provided below:

:::note
If you're feeling confident, you can use the code from the previous part and use 
the comments marked as "TODO" to follow each step instead of 
pasting in the helper file for this part.
:::

```rust
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
  use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::{
		sp_runtime::traits::Hash,
		traits::{ Randomness, Currency, tokens::ExistenceRequirement },
		transactional
	};
	use sp_io::hashing::blake2_128;

	#[cfg(feature = "std")]
	use serde::{Deserialize, Serialize};

    // ACTION #1: Write a Struct to hold Kitty information.
    
    // ACTION #2: Enum declaration for Gender.

    // ACTION #3: Implementation to handle Gender type in Kitty struct.


    #[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types it depends on.
    #[pallet::config]
    pub trait Config: pallet_balances::Config + frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        
		/// The Currency handler for the Kitties pallet.
		type Currency: Currency<Self::AccountId>;

        // ACTION #5: Specify the type for Randomness we want to specify for runtime.
    }

    // Errors.
    #[pallet::error]
    pub enum Error<T> {
        // TODO Part III
    }

    // Events.
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // TODO Part III
    }

    #[pallet::storage]
    #[pallet::getter(fn all_kitties_count)]
    pub(super) type AllKittiesCount<T: Config> = StorageValue<_, u64, ValueQuery>;
    
    // ACTION #6: Add Nonce storage item.

    // ACTION #9: Remaining storage items.

    // TODO Part IV: Our pallet's genesis configuration.

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        
        // TODO Part III: create_kitty
        
        // TODO Part III: set_price
        
        // TODO Part III: transfer

        // TODO Part III: buy_kitty
        
        // TODO Part III: breed_kitty
    }

    // ACTION #4: helper function for Kitty struct

    impl<T: Config> Pallet<T> {
        // TODO Part III: helper functions for dispatchable functions
        
        // ACTION #7: increment_nonce helper

        // ACTION #8: random_hash helper

        // TODO: mint, transfer_from
        
    }
}
```


#### A. What information to include

Structs are a useful tool to help store data that have things in common. 
For our purposes, our Kitty will carry multiple traits which we can store in a single struct 
instead of using separate storage items. This comes in handy when trying to optimize
for storage reads and writes because our runtime will need to perform less read/writes to update
multiple values. Read more about storage best practices [here][storage-best-practice-kb]. 

Let's first go over what information a single Kitty will carry:

- **`dna`**: the hash used to identify the DNA of a Kitty, which corresponds to its unique features.
  DNA is also used to breed new Kitties and to keep track of different Kitty generations.
- **`price`**: this is a balance that corresponds to the amount needed to buy a Kitty and
  set by its owner.
- **`gender`**: an enum that can be either `Male` or `Female`.
- **`owner`**: an account ID designating a single owner.

#### B. Sketching out the types held by our struct

Looking at the items of our struct from [step 1A](/docs/Tutorials/Kitties/Part%201/create-kitties#a-what-information-to-include), we can deduce the following types: 

- **`[u8; 16]`** for `dna` (to use 16 bytes to represent a Kitty's DNA)
- **`BalanceOf`** for `price` (this is a custom type using FRAME's [`Currency` trait][currency-frame])
- **`Gender`** for `gender` (we're going to need to create this!)

First, we'll need to add in our custom types for `BalanceOf` and `AccountOf` before we declare our struct. Replace ACTION #1 with the following snippet:

```rust
type AccountOf<T> = <T as frame_system::Config>::AccountId;
type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
```

Now, paste in the our Kitty struct:

```rust
// Struct for holding Kitty information.
#[derive(Clone, Encode, Decode, PartialEq)]
pub struct Kitty<T: Config> {
    pub dna: [u8; 16],   
    pub price: Option<BalanceOf<T>>,
    pub gender: Gender,
    pub owner: AccountOf<T>,
}
```

Notice how we use the derive macro to include [various helper traits][prelude-traits-rustdocs] for using our struct.
For type `Gender`, we'll need to build out our own custom enum and helper functions. Now's a good time
to do that.

### 2. Writing a custom type for `Gender`

We've just created a struct that requires a custom type called `Gender`. This type will handle an enum defining our Kitty's gender. To create it, you'll build out the following parts:

- **An enum declaration**, which specifies `Male` and `Female` values.
- **A function to configure a defaut value**, based on the enum.

:::info
Setting up our `Gender` enum using a function to configure its default value will allow us to derive a Kitty's gender by passing in the randomness
created by each Kitty's DNA.
:::

#### A. Enums

Replace ACTION item #2 with the following enum declaration:

```rust
#[derive(Encode, Decode, Debug, Clone, PartialEq)]
pub enum Gender {
    Male,
    Female,
}
```

Notice the use of the [derive macro][derive-macro-rust] which must precede the enum declaration. This wraps our enum in the data structures it will need to interface with other types in our runtime.

Then, we need a function to define a default implementation for our enum by using Rust's [`Default` trait][default-rustdocs].

Replace the line containing ACTION #3 with: 

```rust
impl Default for Gender {
    fn default() -> Self {
        Gender::Male
    }
}
```

💡 **This is like saying:** let's give our enum a special trait that allows us to initialize it to a specific value.

Great, we now know how to create a custom struct and specify its default value. But what about providing
a way for a Kitty struct to be _assigned_ a gender value? For that we need to learn one more thing.

#### B. Configuring functions for our Kitty struct

Configuring a struct is useful in order to pre-define a value in our struct. For example, when setting
a value in relation to what another function returns. In our case we have a similar situation where
we need to configure our Kitty struct in such a way that sets `Gender` according to a Kitty's DNA. 

We'll only be using this function when we get to [creating Kitties](/docs/tutorials/Kitties/Part%201/dispatchables-and-events#2-write-the-create_kitty-dispatchable). Regardless, learn how to write it now and get it out of the way.

When you're implementing the configuration trait for a struct inside a FRAME pallet, you're doing the
same type of thing as implementing some trait for an enum except you're implementing the generic
configuration trait, `Config`. In our case we'll create a public function called `gen_gender` that returns the `Gender` type
and uses a random function to choose between `Gender` enum values. 

Replace ACTION #4 with the following code snippet:

```rust
fn gen_gender() -> Gender {
    let random = T::KittyRandomness::random(&b"gender"[..]).0;
    match random.as_ref()[0] % 2 {
        0 => Gender::Male,
        _ => Gender::Female,
    }
}
```

Now whenever `gen_gender()` is called inside our pallet, it will return a pseudo random enum value for `Gender`.

### 3. Implement on-chain randomness

If we want to be able to tell these Kitties apart, we need to start giving them unique properties!
In the previous step, we've made use of `KittyRandomness` which we haven't actually defined yet. Let's get to it.

We'll be using the [Randomness trait][randomness-rustdocs] from `frame_support` to do this. It will be able to generate a random seed which 
we'll create unique Kitties with as well as breed new ones.

In order to implement the `Randomness` trait for our runtime, we must:

**A. Specify it in our pallet's configuration trait.**

The `Randomness` trait from `frame_support` requires specifying it with a paramater to replace the `Output` and `BlockNumber` generics.
Take a look at [the documentation][randomness-rustdocs] and the source code implementation to understand how this works. For our purposes,
we want the output of functions using this trait to be [`H256`][h256-rustdocs] which you'll notice should already be declared at the
top of your working codebase.

Replace the ACTION #5 line with:

```rust
type KittyRandomness: Randomness<Self::H256, Self::BlockNumber>;
```

**B. Specify it for our runtime.**

Given that we're adding a new type for the configuration of our pallet, we need to tell our runtime about its implementation.
This could come in handy if ever we wanted to change the algorithm that `KittyRandomness` is using, without needing to
modify where it's used inside our pallet.

To showcase this point, we're going to implement `KittyRandomness` by assigning it to an instance of [FRAME's `RandomnessCollectiveFlip`][randomness-collective-flip-frame].
Conveniently, the Node Template already has an instance of the `RandomnessCollectiveFlip` pallet. 
All you need to do is **include the `KittyRandomness` type for your runtime inside `runtime/src/lib.rs`**:

```rust
impl pallet_mykitties::Config for Runtime {
    type Event = Event;
	type KittyRandomness = RandomnessCollectiveFlip; // <-- ACTION: add this line.
}
```

:::tip
Check out this [how-to guide](/docs/pallet-design/randomness) on implementing randomness in case you get stuck.
:::

#### Generating random DNA

Generating DNA is similar to using randomness to randomly assign a gender type. The difference is that we'll be making use of `blake2_128` we imported in the previous part. Replace the ACTION #7 line with:

```rust
fn gen_dna() -> [u8; 16] {
    let payload = (
        T::KittyRandomness::random(&b"dna"[..]).0,
        <frame_system::Pallet<T>>::block_number(),
    );
    payload.using_encoded(blake2_128)
}
```

### 4. Write remaining storage items

#### A. Understanding storage item logic

To easily track all of our kitties, we're going to standardize our logic to use a unique ID as the global key
for our storage items. This means that a single unique key will point to our Kitty object (i.e. the struct we previously declared).

In order for this to work, we need to make sure that the ID for a new Kitty is always unique.
We can do this with a new storage item `Kitties` which will be a mapping from an ID (Hash) to the Kitty object.

With this object, we can easily check for collisions by simply checking whether this storage item already contains a mapping
using a particular ID. For example, from inside a dispatchable function we could check using:

```rust
ensure!(!<Kitties<T>>::exists(new_id), "This new id already exists");
```

:::tip
Our pallet's logic can best be understood
by examining the storage items we'll be using. 
In other words, **the way we define the conditions for reading and writing to our runtime's storage helps us breakdown the items we'll need to enable NFT capabilities**. 
::: 

We care about state transitions and persistance around two main concepts our runtime needs to be made aware of:

1. Unique assets, like currency or Kitties (this will be held by a storgae map called `Kitties`)
2. Ownership of those assets, like account IDs (this will be handled by `KittyCnt` and a new storage map called `KittiesOwned`)

#### B. Using a `StorageMap` 

To create a storage instance for the Kitty struct,
we'll be using [`StorageMap`][storage-map-kb] &mdash; a hash-map provided
to us by FRAME. 

Here's what the `Kitties` storage item looks like:

```rust
#[pallet::storage]
#[pallet::getter(fn kitty)]
pub(super) type Kitties<T: Config> = StorageMap<
    _, 
    Twox64Concat, 
    T::Hash, 
    Kitty<T>
    >;
```

Breaking it down, we declare the storage type and assign a `StorageMap` that takes:

- The [`Twox64Concat`][2x64-rustdocs] hashing algorithm.
- A key of type `T::Hash`.
- A value of type `Kitty<T>`.

The `KittiesOwned` storage item is similar except that we'll be using a `BoundedVec` to keep track of some maximum number of Kitties we'll configure in `runtime/src/lib.s`.

```rust
#[pallet::storage]
#[pallet::getter(fn kitties_owned)]
pub(super) type Kitties<T: Config> = StorageMap<
    _, 
    Twox64Concat, 
    T::AccountId, 
    BoundedVec<T::Hash, T::MaxKittyOwned>,
    ValueQuery
    >;
```

Your turn!
Copy the two code snippets above to replace line ACTION #8. 

Before we can check our pallet compiles, we need to add `MaxKittyOwned`, which is a pallet constant that we need to declare (similar to `KittyRandomness` in the previous steps). Replace ACTION #9 with:

```rust
    #[pallet::constant]
    type MaxKittyOwned: Get<u32>;
```
And add this type to `runtime/src/lib.rs`:

```rust
parameter_types! {              // <- add this macro 
    // One can own at most 9,999 Kitties
    pub const MaxKittyOwned: u32 = 9999; 
    }

/// Configure the pallet-kitties in pallets/kitties.
impl pallet_kitties::Config for Runtime {
    type Event = Event;
    type Currency = Balances;
    type KittyRandomness = RandomnessCollectiveFlip;
    type MaxKittyOwned = MaxKittyOwned; // <- add this line
}
```

Assuming you've followed the above steps correctly, now's a good time to check that your pallet compiles:

```rust
cargo build -p pallet-mykitties
```

Running into difficulties? Check your solution against the [completed helper code](https://github.com/substrate-developer-hub/substrate-how-to-guides/blob/main/static/code/kitties-tutorial/03-dispatchables-and-events.rs) for this part of the tutorial.

:::note Congratulations!
If you've made it this far, you now have the foundations for your pallet to
handle the creation and changes in ownership of your Kitties! In this part of the tutorial, we've learnt:

- How to write a struct and use it in a `StorageMap`.
- How to implement a custom type.
- How to set a default enum value for a custom type.
- How to create a function to set a value for that custom type.
- How to implement the Randomness trait to write a function that generates randomness using a nonce.
- How to write `StorageMap` storage items.
:::

## Next steps

- Create a dispatchable function that mints a new Kitty
- Create a helper function to handle storage updates
- Implement Errors and Events

[default-rustdocs]: https://doc.rust-lang.org/std/default/trait.Default.html
[randomness-rustdocs]: https://substrate.dev/rustdocs/latest/frame_support/traits/trait.Randomness.html
[hash-rustdocs]: https://substrate.dev/rustdocs/latest/sp_runtime/traits/trait.Hash.html
[h256-rustdocs]: https://substrate.dev/rustdocs/latest/sp_core/struct.H256.html
[randomness-collective-flip-frame]: https://substrate.dev/rustdocs/latest/pallet_randomness_collective_flip/index.html
[nonce-rustdocs]: https://substrate.dev/rustdocs/latest/frame_system/struct.AccountInfo.html#structfield.nonce
[2x64-rustdocs]: https://substrate.dev/rustdocs/latest/frame_support/struct.Twox64Concat.html
[prelude-traits-rustdocs]: https://substrate.dev/rustdocs/latest/sp_std/prelude/index.html#traits
[derive-macro-rust]: https://doc.rust-lang.org/reference/procedural-macros.html#derive-macros
[storage-best-practice-kb]: https://substrate.dev/docs/en/knowledgebase/runtime/storage#best-practices
[storage-map-kb]: https://substrate.dev/docs/en/knowledgebase/runtime/storage#storage-map
[storage-value-kb]: https://substrate.dev/docs/en/knowledgebase/runtime/storage#storage-value
[helper-code-pt2]: https://github.com/substrate-developer-hub/substrate-how-to-guides/blob/main/static/code/kitties-tutorial/02-create-kitties.rs
[currency-frame]: https://substrate.dev/rustdocs/latest/frame_support/traits/tokens/currency/trait.Currency.html#associatedtype.Balance
