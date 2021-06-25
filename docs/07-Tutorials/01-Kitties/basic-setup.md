---
sidebar_position: 2
keywords: pallet design, intermediate, runtime
code: code/kitties-tutorial/01-basic-setup.rs
---

# Part I: Basic set-up

:::note
This workshop assumes that you have already installed the prerequisites for building with Substrate on your machine.
If you haven't already, head over to our [installation guide][installation].
:::

## Learning outcomes

:arrow_right: Common basic patterns for Substrate runtime development.

:arrow_right: How to repurpose the Substrate Node Template.

:arrow_right: Create a single value storage item to keep track of a `u64` value.

## Overview

Before we can start making Kitties, we first need to do a little groundwork. This part covers the basic patterns involved with using the Substrate Node Template to set up a custom pallet and include a simple storage item.

## Steps

### 1. Create a pallet and integrate it to your runtime

#### Getting starting with the node template

The [Substrate Node Template][substrate-node-template] provides us with an "out-of-the-box" blockchain node. Our biggest advantage
is that the networking and consensus layers are already built and all we need to focus on is building out
our runtime logic. Start by cloning the node template:

```bash
git clone git@github.com:substrate-developer-hub/substrate-node-template.git
```

Using your IDE, go ahead and rename the template node by modifying the details in the **`/node/Cargo.toml`** file.
For our purposes, what's important is to:

- rename `node-template` to `substratekitties`
- rename `node-template-runtime` to `kitties-runtime`

And update your **`runtime/Cargo.toml`** file accordingly:

- rename `node-template-runtime` to `kitties-runtime`

:::tip Use the side panel as a scratch pad! [**coming soon**]
Each part will have incomplete code with comments to guide you on completing it. Make sure to only use it as a scratch-pad
and copy it to your IDE &mdash; it doesn't save your work!
:::

#### Creating and integrating `pallet_kitties`

Now that your node template is ready, we can proceed to creating our pallet.

[Pallets][pallets-kb] in Substrate are used to define runtime logic. In our case, we'll be creating a single pallet that manages all of the
logic of our Substrate Kitties dApp. The node template already comes with a template pallet and folder structure that we can re-use:

```bash
substratekitties
|
+-- node
|
+-- pallets
|   |
|   +-- template           <-- Rename to `kitties`
|       |
|       +-- Cargo.toml     <-- *Modify* this file
|       |
|       +-- src
|           |
|           +-- lib.rs     <-- *Remove* contents
|           |
|           +-- mock.rs    <-- *Remove* contents
|           |
|           +-- tests.rs   <-- *Remove* contents
```

Go ahead and remove all the contents of `lib.rs` as well as `mock.rs` and `tests.rs`.

All of our pallet's logic will live inside `lib.rs`. **We'll
be using `mock.rs` and `tests.rs` towards to end of this tutorial when we write unit tests for our dApp.**

At this point, we're in a good place to lay out the basic structure of our pallet, after which we can check if our node builds without error. By structure, we're talking about outlining the parts inside the `lib.rs` file of our newly created `pallet_kitties`.

Every FRAME pallet has:

- a set of `frame_support` and `frame_system` dependencies
- required [attribute macros][macros-kb] (i.e. configuration traits, storage items, hooks and function calls).

:::note
We'll be updating the dependencies as needed by the code we write.
:::
In its most bare-bones version, a pallet looks like this:

```rust
pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]

    #[pallet::hooks]

    #[pallet::call]
    impl<T: Config> Pallet<T> {}
}
```

:::info
Refer to [this guide](./01-basics/basic-pallet-integration) to learn the basic pattern for integrating a new pallet to your runtime and
read more about pallets in this [knowledgebase article][pallets-kb].  
:::

Now that we have a pallet called `pallet_kitties` we must implement it for our runtime. Since we haven't yet
defined anything in our pallet, our `Config` implementation is pretty simple.

In `runtime/lib.rs` include this
line after the other trait implementations:

```rust
impl pallet_kitties::Config for Runtime {}
```

With all of that done, we're geared up to test whether everything works.

Run the following command to build and launch our chain. This can take a little while depending on your machine:

```bash
cargo build --release
```

Assuming everything compiled without error, we can launch our chain and check that it is producing blocks:

```bash
./target/release/substratekitties --tmp --dev
```

Works? If not, make sure you've followed the steps to integrate your new pallet to your runtime. If yes, fantastic!

You don't need to keep your node running. This was just a way to check that your pallet and runtime are
properly configured. In the next steps we will start writing the storage items our Kitty dApp will require.

### 2. Include a storage item to track all Kitties

Let's start by adding the most simple logic we can to our runtime: a function which stores a variable in runtime.

To do this we'll use [`StorageValue`][storagevalue-rustdocs] from Substrate's [storage API][storage-api-rustdocs] which is a trait that depends
on the storage macro.

All that means for our purposes is that for any storage item we want to declare, we must include `#[pallet::storage]` beforehand. Using `StorageValue` as an example, this would look like this:

```rust
#[pallet::storage]
#[pallet::getter(fn get_storage_value)]
pub(super) type SomeStorageValue <T: Config> StorageValue <
    _,
    u64,
    ValueQuery,
>
```

With that declared, we can use the various functions from Substrate's storage API to read and write to
storage. For example, using `get()` and `put()` would look like:

```rust
    // Get value in storage using the getter function.
    let storage_value = Self::get_storage_value();

    // Another way to get the value.
    let storage_value = <SomeStorageValue<T>>::get();

    // Write to storage.
	<SomeStorageValue<T>>::put(0u64);
```

:::tip Your turn!
Our Kitties dApp will need to keep track of a number of things. The first will be the number of Kitties.
Write a storage item to keep track of all Kitties, call it `AllKittiesCount`.
:::

### 3. Build and check your pallet

From the previous step, your pallet should contain a storage item called `AllKittiesCount` which keeps track of a
single `u64` value. As part of the basic setup, we're doing great!

:::info
As mentioned in the [overview of this workshop](overview),
you'll be implementing a total of 9 storage items which you'll discover as you
write out your pallet's logic in the next parts.
:::

Before we move on, let's make sure everything compiles. We don't need to rebuild our entire node each time we update our pallet.
Instead, we can use a command to only build our pallet. To do this, run the following command from inside your pallet directory:

```bash
cargo build -p pallet-kitties
```

It should compile error-free. If not, go back and check that all the macros are in place and that you've included the
FRAME dependencies.

:::note Congratulations!
 You've completed the first part of this workshop. At this stage, you've learnt the various patterns for:

- customizing the Substrate node template and including a custom pallet
- building a Substrate chain and checking that a target pallet compiles
- declaring and using a `u64` storage item
:::

## Next steps

- Writing a struct in a `StorageMap` to store details about our Kitties
- Using the Randomness trait to create unique Kitties
- Creating our pallet's remaining storage items

[installation]: https://substrate.dev/docs/en/knowledgebase/getting-started/
[substrate-node-template]: https://github.com/substrate-developer-hub/substrate-node-template
[pallets-kb]: https://substrate.dev/docs/en/knowledgebase/runtime/pallets
[macros-kb]: https://substrate.dev/docs/en/knowledgebase/runtime/macros#frame-v2-macros-and-attributes
[storagevalue-rustdocs]: https://substrate.dev/rustdocs/v3.0.0/frame_support/storage/trait.StorageValue.html
[storage-api-rustdocs]: https://substrate.dev/rustdocs/v3.0.0/frame_support/storage/index.html
