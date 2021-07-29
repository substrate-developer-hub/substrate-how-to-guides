---
sidebar_position: 3
keywords: pallet design, intermediate, runtime
code: code/kitties-tutorial/02-create-kitties.rs
---

# Part II: Uniqueness, custom types and storage maps

_In this part of the tutorial, we'll build out the components of our pallet
needed to manage the creation and ownership of our Kitties._

## Learning outcomes

:arrow_right: Writing a custom struct and using it in storage.

:arrow_right: Creating a custom type and implementing it for your pallet's `Config` trait.

:arrow_right: Using the Randomness trait in a helper function.

:arrow_right: Adding and using `StorageMap` items to a pallet.

## Overview

This part of the tutorial dives into some pillar concepts for developing pallets with FRAME. Ontop of learning
how to use existing types and traits, you'll learn how create your own (_FRAME doesn't have a concept of Gender!_). At the end
of this part, you will have implemented all remaining storage items according to the logic outlined for the Substrate Kitty dApp
[in the overview of this tutorial](/overview).

## Steps

### 1. Create the Kitty struct

#### A. What information to include

Structs are a useful tool to help carry information that corresponds to the same object.
For our purposes, instead of using separate storage items for each of our Kitty traits,
we can store this information in a single struct. This comes in handy when trying to optimize
for storage reads and writes. Let's first go over what information a single Kitty carries:

- **`id`**: a unique hash to identify each Kitty.
- **`dna`**: the hash used to identify the DNA of a Kitty, which corresponds to its unique features.
  DNA is also used to breed new Kitties and to keep track of different Kitty generations.
- **`price`**: this is a balance that corresponds to the amount needed to buy a Kitty and
  determined by its owner.
- **`gender`**: an enum that can be either `Male` or `Female`.

#### B. Writing out the struct

Now that we know what to include in our Kitty struct, let's recap the basics of declaring a struct.

A struct has the following format:

```rust
#[derive(Clone, Encode, Decode, Default, PartialEq)]
pub struct SomeStruct<SomeType1, SomeType2> {
    value_1: SomeType1,
    value_2: SomeType2,
}
```

We use the derive macro to include [various helper traits][prelude-traits-rustdocs] for using our struct.

#### C. Adding the `Hash` dependency

Looking at the items of our struct from [step 1A](/docs/Tutorials/Kitties/create-kitties#a-what-information-to-include), we can deduce the following types: **`Hash`** for `id` and `dna`; **`Balance`** for `price`; and **`Gender`** for `gender`.

For `Gender`, we'll need to build out our own custom enum and helper functions. Similarly, `Balance`
will be something we'll later define in our configuration trait. However, we can start by importing
the `Hash` types from the `frame_support` library to satisfy `id` and `dna`.

Write this at the top of your pallet where
the dependencies are declared:

```rust
	use frame_support::sp_runtime::traits::{Hash};
```

In doing so, you've given your pallet access to Substrate's [`Hash`][hash-rustdocs] type which will be the type for your Kitty ID and DNA.

:::tip Your turn!
Now that you know the different types your `Kitty` struct will require, start writing it!

Your struct declaration will look like this: `pub struct Kitty<Hash, Balance> {}` and include the fields we went over in the previous section.

**HINT**: Have a look at [this how-to guide](../../pallet-design/storage-value-struct) in case you get stuck.
:::

#### D. Adding the custom `Gender` dependency

We've just created a struct that requires a custom type called `Gender`. This type will handle an enum defining our Kitty's gender. To create it, you'll build out the following parts:

1. **An enum declaration**, which specifies `Male` and `Female` values.
2. **A function to configure a defaut value**, based on the enum.

:::info
Setting up our `Gender` enum using a function to configure its default value will allow us to derive a Kitty's gender by passing in the randomness
created by each Kitty's DNA.
:::

#### E. Enums

Writing enums requires us to use the [derive macro][derive-macro-rust] which must precede the enum declaration.

A typical enum would be structured as such:

```rust
	#[derive(Encode, Decode, Debug, Clone, PartialEq)]
	pub enum TypicalEnum {
		One,
		Two,
	}
```

Then, we can write out the default implementation by using Rust's [`Default` trait][default-rustdocs].
This would look something like this:

```rust
	impl Default for TypicalEnum {
		fn default() -> Self {
			TypicalEnum::One
		}
	}
```

> 💡 **It's like saying:** let's give our enum a special trait that allows us to initialize it to a specific value.

Great, we now know how to create a custom struct and specify its default value. But what about providing
a way for a Kitty struct to be assigned a gender value? For that we need to learn one more thing.

#### F. Configuring functions for our Kitty struct

Configuring a struct is useful in order to pre-define a value in our struct. For example, if we want to
set a value according to what another function returns. In our case we have a similar situation where
we need to configure our Kitty struct in such a way that sets `Gender` according to a Kitty's DNA. We'll
only be using this function when we get to creating Kitties but let's first learn how to write it and get it out
of the way.

When you're implementing the configuration trait for a struct inside a FRAME pallet, you're doing the
same type of thing as implementing some trait for an enum except you're implementing the generic
configuration trait, `Config`.

Building off our previous example, this would look like:

```rust
impl<T: Config> TypicalEnum {
		pub fn special_function(some_hash: T::Hash) -> TypicalEnum {
			if some_hash.as_ref()[0] % 2 == 0 {
				TypicalEnum::One
			} else {
				TypicalEnum::Two
			}
		}
	}
```

Now whenever `special_function` is called inside our pallet, it will return the enum value associated with
the function's logic.

:::tip Your turn!
Write out the `Gender` enum, trait implementations and `Config` implementation to define the behaviour
of `Gender` for our `Kitties` struct.

**Hint**: reuse the logic in `special_function` &mdash; that's how we'll retrieve the gender of our Kitties!
:::

### 2. Implement on-chain randomness

If we want to be able to tell these Kitties apart, we need to start giving them unique properties!
For our dApp, we need to generate a unique ID for each Kitty and some random DNA.

We'll be using the [Randomness trait][randomness-rustdocs] from `frame_support` to generate a random seed which we'll need
to breed and create new unique Kitties. Alongside this trait, we'll also need a nonce which we'll create as a separate function
and a hashing function which we'll get by using the [`Hash` trait][hash-rustdocs] that we imported in [step 1C](/docs/Tutorials/Kitties/create-kitties#c-adding-the-hash-dependency).

Here's what our random hashing function looks like:

```rust
        fn random_hash(sender: &T::AccountId) -> T::Hash {
            let nonce = <Nonce<T>>::get();
            let seed = T::Randomness::random_seed();

            T::Hashing::hash_of(&(seed, &sender, nonce))
        }
```

In order to implement the `Randomness` trait for our runtime, we must:

**A. Specify it in our pallet's configuration trait.**

The `Randomness` trait from `frame_support` requires specifying it with a paramater to replace the `Output` generic.
Take a look at the documentation and the source code implementation to understand how this works. For our purposes,
we want the output of functions using this trait to be [`H256`][h256-rustdocs]:

```rust
type KittyRandomness: Randomness<H256>;
```

**B. Specify it for our runtime.**

Given that we're adding a new type for the configuration of our pallet, we need to tell our runtime about its implementation.
This could come in handy if ever we wanted to change the algorithm that `KittyRandomness` is using, without needing to
modify where it's used inside our pallet.

To showcase this point, we're going to implement `KittyRandomness` by assigning it to an instance of [FRAME's `RandomnessCollectiveFlip`][randomness-collective-flip-frame].
This requires you to integrate the `RandomnessCollectiveFlip` pallet to your runtime and implement it. Once you do that, inside your
`runtime/src/lib.rs` file, include the `KittyRandomness` type for your runtime:

```rust
impl pallet_kitties::Config for Runtime {
	type KittyRandomness = RandomCollectiveFlip;
}
```

:::tip
Check out this [how-to guide](/docs/pallet-design/randomness) on implementing randomness in case you get stuck.
:::

#### Nonce

:::note why our random hashing function needs a nonce

Our goal is to create as much entropy as we can for `hash_of` to produce enough randomness when creating Kitty IDs and DNA.
Since `random_seed()` does not change for multiple transactions in the same
block, and since it may not even generate a
random seed for the first 80 blocks, we need to create a nonce for our pallet to manage and use in our private `random_hash` function.  
:::

We'll use the nonce provided by [`frame_system::AccountInfo`][nonce-rustdocs] and create a storage item to keep track of it as we modify it.

So we'll need to do a couple things.

First, create a storage item for the nonce value:

```rust
	#[pallet::storage]
    #[pallet::getter(fn get_nonce)]
    pub(super) type Nonce<T: Config> = StorageValue<_, u64, ValueQuery>;
```

Second, create a function that increments the nonce:

```rust
fn increment_nonce() -> DispatchResult {
            <Nonce<T>>::try_mutate(|nonce| {
                let next = nonce.checked_add(1).ok_or(Error::<T>::NonceOverflow)?;
                *nonce = next;

                Ok(().into())
            })
        }
```

:::tip Your turn!

Feel free to use the code snippets above. Make sure to
include the `Nonce` storage item among the section with your other storage items, and the
`increment_nonce()` function in the helper function section.
:::

#### Unique Kitty ID

To easily track all of our kitties, it would be helpful to standardize our logic to use a unique id as the global key
for our storage items. This means that a single unique key will point to our Kitty object, and all other links to ownership
will point to that key.

The ID on the Kitty object will serve that purpose, but we need to make sure that the ID for a new Kitty is always unique.
We can do this with a new storage item `Kitties` which will be a mapping from an ID (Hash) to the Kitty object.

With this object, we can easily check for collisions by simply checking whether this storage item already contains a mapping
using a particular ID. For example, from inside a dispatchable function we could check using:

```rust
ensure!(!<Kitties<T>>::exists(new_id), "This new id already exists");
```

You'll be implementing the `StorageMap` for `Kitties` in the next section.

### 3. Include storage items

We'll be needing a total of 9 storage items
for our Kitty pallet. We already included `Nonce` and we've already created the basis for
our Kitty object &mdash; we just need to implement a way to keep track of them now!

Our pallet's logic can best be understood
by examining the storage items we'll be using. In other words, the way we define the conditions
for reading and writing to our runtime's storage
helps us breakdown the items we'll need to enable NFT capabilities. In our case, we care about state transitions and persistance around two main concepts our runtime needs to be made aware of:

1. unique assets, like currency or Kitties
2. helper datastructures, like the nonce, counters or account maps

This already starts to lay the foundations for our Kitty pallet logic. But there's an important layer beneath these two concepts: our runtime needs to have a sense of asset ownership as well as the ability to keep track of changes in ownership and owned quantities.

In our application, ontop of keeping a single storage instance for Kitty objects, we need a number of storage items to keep track of:

- Kitties in existence
- Owned Kitties

:::note The overarching pattern here is to keep track of _who_ and _what_.

This boils down to the following storage items (in addition to `Kitties` and `Nonce`):

**Tracking ownership**

- `<KittyOwner<T>>`: Keeps track of what accounts own what Kitty.
- `<OwnedKittiesArray<T>>`: Keep track of who a Kitty is owned by.
- `<OwnedKittiesCount<T>>`: Keeps track of the total amount of Kitties owned.
- `<OwnedKittiesIndex<T>>`: Keeps track of all owned Kitties by index.

**Tracking existing Kitties**

- `<AllKittiesArray<T>>`: An index to track of all Kitties.
- `<AllKittiesCount<T>>`: Stores the total amount of Kitties in existence.
- `<AllKittiesIndex<T>>`: Keeps track of all the Kitties.
  :::

#### Using a `StorageMap`

Every storage item declaration will follow a similar
pattern as when we wrote the storage item for `Nonce`. The only difference is which data structure type each storage item requires.

To create a storage instance for the Kitty struct,
we'll be using `StorageMap` &mdash; a hash-map provided
to us by FRAME. This differs from the storage instance we created for
`Nonce` which, because we want it to keep track of a single `u64` value, we used `StorageValue`. Here, we need our storage to keep track
of the mapping hash IDs and Kitty objects.

Here's the basic pattern for declaring a storage map storage item, showing the storage for Kitty objects as an example:

```rust
	#[pallet::storage]
    #[pallet::getter(fn kitty)]
    pub(super) type Kitties<T: Config> =
        StorageMap<_, Twox64Concat, T::Hash, Kitty<T::Hash, T::Balance>, ValueQuery>;
```

Breaking it down, we declare the storage type and assign a `StorageMap` that takes:

- The [`Twox64Concat`][2x64-rustdocs] hashing algorithm.
- A key of type `T::Hash`.
- A value of type `Kitty<T::Hash, T::Balance>`.

:::tip Your turn!
Use the storage items outlined above to help you finish writing the remaining storage items. Follow the same pattern
we used for `Nonce` and `Kitties` &mdash; just remember what type each item is meant to store!

**HINT**: Remember to include a getter function for each storage item &mdash; except those handling indices) .
:::

Assuming you've finished implementing all of your storage items, now's a good time to check that your pallet compiles correctly:

```rust
cargo build -p pallet-kitties
```

:::note Congratulations!
If you've made it this far, you now have the foundations for your pallet to
handle the creation and ownership of your Kitties! In this part of the tutorial, we've learnt:

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
- Create and use Events

[default-rustdocs]: https://doc.rust-lang.org/std/default/trait.Default.html
[randomness-rustdocs]: https://substrate.dev/rustdocs/v3.0.0/frame_support/traits/trait.Randomness.html
[hash-rustdocs]: https://substrate.dev/rustdocs/v3.0.0/sp_runtime/traits/trait.Hash.html
[h256-rustdocs]: https://substrate.dev/rustdocs/v3.0.0/sp_core/struct.H256.html
[randomness-collective-flip-frame]: https://substrate.dev/rustdocs/v3.0.0/pallet_randomness_collective_flip/index.html
[nonce-rustdocs]: https://substrate.dev/rustdocs/v3.0.0/frame_system/struct.AccountInfo.html#structfield.nonce
[2x64-rustdocs]: https://substrate.dev/rustdocs/v3.0.0/frame_support/struct.Twox64Concat.html
[prelude-traits-rustdocs]: https://substrate.dev/rustdocs/v3.0.0/sp_std/prelude/index.html#traits
[derive-macro-rust]: https://doc.rust-lang.org/reference/procedural-macros.html#derive-macros
