---
sidebar_position: 2
keywords: pallet design, intermediate, runtime
---

# Basic set-up

:::note
This tutorial assumes that you have already installed the prerequisites for building with
Substrate on your machine. If you haven't already, head over to our [installation guide]
[installation].
:::

## Overview

Before we can start making Kitties, we first need to do a little groundwork. This part covers the
basic patterns involved with using the Substrate Node Template to set up a custom pallet and
include a simple storage item.

## Learning outcomes

:arrow_right: &nbsp; Renaming a Substrate Node Template using the Kickstart tool.

:arrow_right: &nbsp; Basic patterns for building and running a Substrate node.

:arrow_right: &nbsp; Create a storage item to keep track of a single `u64` value.

## Steps

### 1. Set-up Your Template Node

The [Substrate Node Template][substrate-node-template] provides us with an "out-of-the-box"
blockchain node. Our biggest advantage in using it are that both networking and consensus layers
are already built and all we need to focus on is building out our [runtime][runtime-kb] and [pallet]
[pallets-kb] logics. Before we get there, we need to set-up our project in terms of naming and
dependencies.

We'll use a CLI tool called [kickstart][kickstart-tool] to easily rename our node template.

In the root directory of your local workspace, run the following command:

```bash
kickstart https://github.com/sacha-l/kickstart-substrate
```

This command will clone a copy of the most recent Node Template and ask how you would like to call
your node and pallet. Type in:

- `kitties` - as the name of our node. The node will be named as `node-kitties`.
- `kitties` - as the name of your pallet. The pallet will be named as `pallet-kitties`.

This will create a directory called `kitties` with a copy of the [Substrate Node Template]
[substrate-node-template] containing the name changes that correspond our template node, runtime
and pallet.

Open the `kitties` directory in your favorite code editor and rename it to `kitties-tutorial`.
Renaming this directory will be helpful once you start creating other projects with the node
template &mdash; it'll help keep things organized!

:::note
Notice the directories that the `kickstart` command modified:

- **`/node/`** - This contains all the logic that allows your node to interact with your runtime and
    RPC clients.
- **`/pallets/`** - Here's where all your custom pallets live.
- **`/runtime/`** - This is where all pallets (both custom "internal" and "external" ones) are
    aggregated and implemented for the chain's runtime.
:::

By default, you'll notice that the instance of our modified template pallet name remains
`TemplateModule`. Change it to `SubstrateKitties` (in `runtime/src/lib.rs`):

```rust
construct_runtime!(
    pub enum Runtime where
    Block = Block,
    NodeBlock = opaque::Block,
    UncheckedExtrinsic = UncheckedExtrinsic
    {
        // --snip
        SubstrateKitties: pallet_kitties::{Pallet, Call, Storage, Event<T>},
    }
);
```

We can already build the node as is by navigating to directory `kitties` in terminal and running
this command:

```bash
cargo build --release
```

It's normal if this command takes a little while depending on your machine &mdash; it's building a
whole bunch of crates from the Substrate crates and libraries. The nice thing is that once we run
this command the first time, it won't need to rebuild all the crates when we build subsequent
times.

Assuming that your node builds successfully, launch it in development mode to make sure it works:

```bash
./target/release/node-kitties --tmp --dev
```

You should see blocks being created in your terminal. The `--tmp` and `--dev` flags mean we're
running a temporary node in development mode.

### 2. Write Out `pallet_kitties` Scaffold

We'll be spending most of this tutorial in the `pallets` directory of our template node. Let's take
a glance at the folder structure in our workspace:

```
kitties-tutorial           <--  The name of our project directory
|
+-- node
|
+-- pallets
|   |
|   +-- kitties
|       |
|       +-- Cargo.toml
|       |
|       +-- src
|           |
|           +-- lib.rs     <-- Remove contents
|           |
|           +-- mock.rs    <-- Remove file
|           |
|           +-- tests.rs   <-- Remove file
|
+-- Cargo.toml
```

> You can go ahead and remove `mock.rs` and `tests.rs`. **We won't be learning about
> using these in this tutorial. Have a look at [this how-to guide](/docs/testing/test-transfer) if
> you're curious to learn how testing works.**

[Pallets][pallets-kb] in Substrate are used to define runtime logic. In our case, we'll be creating
a single pallet that manages all of the logic of our Substrate Kitties application.

Let's lay out the basic structure of our pallet by outlining the parts inside the
`pallets/kitties/src/lib.rs`.

:::note
Our pallet's directory `pallets/kitties/` is not the same as our pallet's name. The name
   of our pallet as Cargo understands it is `pallet-kitties`.
:::

Every FRAME pallet has:

- A set of `frame_support` and `frame_system` dependencies.
- Required [attribute macros][macros-kb] (i.e. configuration traits, storage items and function
  calls).

:::note
We'll be updating additional dependencies as we progress through the coming parts of this
   tutorial.
:::

Here is the most bare-bones version of the Kitties pallet we will be building in this tutorial. It
contains the starting point for adding code for coming sections of this tutorial, with comments
marked with **TODO** to indicate code we will be writing later, and **ACTION** to indicate code
that will be written in the current part of the tutorial.

Paste the following code in `/pallets/kitties/src/lib.rs`:

```rust
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{sp_runtime::traits::{Hash, Zero},
                        dispatch::{DispatchResultWithPostInfo, DispatchResult},
                        traits::{Currency, ExistenceRequirement, Randomness},
                        pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_core::H256;

    // TODO Part II: Struct for holding Kitty information.

    // TODO Part II: Enum and implementation to handle Gender type in Kitty struct.

    #[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types it depends on.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The Currency handler for the Kitties pallet.
        type Currency: Currency<Self::AccountId>;

        // TODO Part II: Specify the custom types for our runtime.

    }

    // Errors.
    #[pallet::error]
    pub enum Error<T> {
        // TODO Part III
    }

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // TODO Part III
    }

    // ACTION: Storage item to keep a count of all existing Kitties.

    // TODO Part II: Remaining storage items.

    // TODO Part III: Our pallet's genesis configuration.

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        // TODO Part III: create_kitty

        // TODO Part III: set_price

        // TODO Part III: transfer

        // TODO Part III: buy_kitty

        // TODO Part III: breed_kitty
    }

    // TODO Parts II: helper function for Kitty struct

    impl<T: Config> Pallet<T> {
        // TODO Part III: helper functions for dispatchable functions

        // TODO: increment_nonce, random_hash, mint, transfer_from

    }
}
```

Now try running the following command to rebuild your chain:

```bash
cargo build --release
```

Get an error about dependencies? That's normal! Our pallet is using `sp-io` which isn't part of the
node template so we must specify it ourselves. In `pallets/kitties/Cargo.toml`, add the
following:

```TOML
[dependencies.sp-io]
default-features = false
git = 'https://github.com/paritytech/substrate.git'
tag = 'monthly-2021-08'
version = '4.0.0-dev'
```

:::warning 
Check that you're using the correct `monthly-*` tag and version otherwise you will get a
   dependency error. Here, we're using `monthly-2021-08` tag of Substrate.
:::

Now run `cargo build --release` again to make sure it builds without errors.

:::note
You will notice the Rust compiler giving you warnings about unused imports. That's fine!
   Just ignore them &mdash; we're going to use those imports later in this tutorial.
:::

In the next step we will include the first storage item for our Kitty application.

### 3. Include a Storage Item to Track All Kitties

Let's start by adding the most simple logic we can to our runtime: a function which stores a
variable in runtime.

To do this we'll use [`StorageValue`][storagevalue-rustdocs] from Substrate's [storage API]
[storage-api-rustdocs] which is a trait that depends on the [storage macro][storage-macro-kb].

All that means for our purposes is that for any storage item we want to declare, we must include the
`#[pallet::storage]`  macro beforehand. Learn more about declaring storage items [here](https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items).

In `pallets/kitties/src/lib.rs`, replace the ACTION line with:

```rust
    #[pallet::storage]
    #[pallet::getter(fn kitty_cnt)]
    /// Keeps track of the number of Kitties in existence.
    pub(super) type KittyCnt<T: Config> = StorageValue<_, u64, ValueQuery>;
```

This creates a storage item for our pallet to keep track of the total count of Kitties in
existence.

### 4. Build Pallet

From the previous step, your pallet should contain a storage item called `KittyCnt` which keeps
track of a single `u64` value. As part of the basic setup, we're doing great!

:::info
As mentioned in the [overview of this tutorial series](../overview), you'll be implementing 3
   storage items in total which you'll discover as you write out your pallet's logic in the next
   parts.
:::

Before we move on, let's make sure everything compiles. We don't need to rebuild our entire node
each time we update our pallet. Instead, we can use a command that only builds our pallet. From
inside your pallet directory, run the following:

```bash
cargo build --release -p pallet-kitties
```

Does your pallet compile without error? Well done if it does! If not, go back and check that all the
macros are in place and that you've included the FRAME dependencies.

:::info
Congratulations! You've completed the first part of this series. At this stage, you've
   learnt the various patterns for:

- Customizing the Substrate node template and including a custom pallet.
- Building a Substrate chain and checking that a target pallet compiles.
- Declaring a single value `u64` storage item.
:::

## Next Steps

- Writing a struct in a `StorageMap` to store details about our Kitties
- Using the Randomness trait to create unique Kitties
- Creating our pallet's remaining storage items

[installation]: https://substrate.dev/docs/en/knowledgebase/getting-started/
[substrate-node-template]: https://github.com/substrate-developer-hub/substrate-node-template
[pallets-kb]: https://substrate.dev/docs/en/knowledgebase/runtime/pallets
[macros-kb]: https://substrate.dev/docs/en/knowledgebase/runtime/macros#frame-v2-macros-and-attributes
[storagevalue-rustdocs]: https://substrate.dev/rustdocs/latest/frame_support/storage/trait.StorageValue.html
[storage-api-rustdocs]: https://substrate.dev/rustdocs/latest/frame_support/storage/index.html
[template-code]: https://github.com/substrate-developer-hub/substrate-how-to-guides/tree/main/static/code/kitties-tutorial
[runtime-kb]: https://substrate.dev/docs/en/knowledgebase/runtime/
[kickstart-tool]: https://github.com/Keats/kickstart
[storage-macro-kb]: https://substrate.dev/docs/en/knowledgebase/runtime/macros#palletstorage
