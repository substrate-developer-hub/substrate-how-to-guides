---
sidebar_position: 2
keywords: pallet design, intermediate, runtime
code: code/kitties-tutorial/01-basic-setup.rs
---

# Part I: Basic set-up

:::note
This tutorial assumes that you have already installed the prerequisites for building with Substrate on your machine.
If you haven't already, head over to our [installation guide][installation].
:::

## Learning outcomes

:arrow_right: How to rename the Substrate Node Template for a custom project.

:arrow_right: Basic patterns for building and running a Substrate node.

:arrow_right: How to create a storage item to keep track of a single `u64` value.

## Overview

Before we can start making Kitties, we first need to do a little groundwork. This part covers the basic patterns involved with using the Substrate Node Template to set up a custom pallet and include a simple storage item.

## Steps

### 1. Get started with the node template

The [Substrate Node Template][substrate-node-template] provides us with an "out-of-the-box" blockchain node. Our biggest advantage
in using it are that both networking and consensus layers are already built and all we need to focus on is building out
our runtime logic. Before we get there, we need to set-up our project. Start by cloning the node template:

```bash
git clone https://github.com/substrate-developer-hub/substrate-node-template.git
```

Rename the project's parent directory to `my-substratekitties` (previously called `substrate-node-template`)
and open `my-substratekitties` in your favorite code editor. Among the folders in the template, the ones we care about are:
- **`/node/`** - This contains all the logic that allows your node to interact with your runtime and RPC clients.
- **`/pallets/`** - Here's where all your custom pallets live.
- **`/runtime/`** - This is where all pallets (both custom "internal" and "external" ones) are aggregated and implemented for the chain's runtime. 

#### Renaming for our node and runtime
Before we start writing any code, let's modify our node template to rename our node and runtime to what we'd like to call it. 

The easiest way to get this done is to use the "search and replace" functionality of your IDE. 

- **Replace the 2 instances of `node-template` with `kitties-node`.** Specifically:
    - **In `/node/Cargo.toml`**: the name for `[[bin]]`; and the name for `[package]`.

- **Replace all instances of `node-template-runtime` with `kitties-runtime`.** Specifically:
    - **In `/node/Cargo.toml`** in 2 places: (1) the path for `runtime-benchmarks` and (2) the dependency name for `path = '../runtime'`.
    - **In `/runtime/Cargo.toml`** in 1 place: the name under `[package]`.
- **Replace all instances of `node_template_runtime` with `kitties_runtime`.** Specifically:
    - **In `chain_spec.rs` in 1 place.**
    - **In `commands.rs` in 2 places.**
    - **In `service.rs` in 3 places.** 

#### Renaming for our pallet
The node template already comes with a template pallet and folder structure that we can re-use. Every pallet 
is contained in a folder, typically named after the pallet, which contains a `Cargo.toml` and a `src` folder. The `src` folder 
is typically made up of a `lib.rs` file and any additional helper 
files. All of the pallet's logic lives in its corresponding `lib.rs` file. 

We're going to need to update information about our pallet too. Here's a glance at the folder structure we're working with to help 
visualize what we'll need to do: 

```bash
my-substratekitties
|
+-- node
|
+-- pallets
|   |
|   +-- template           <-- 1. *Rename* to `kitties`
|       |
|       +-- Cargo.toml     <-- 2. *Modify* this file
|       |
|       +-- src
|           |
|           +-- lib.rs     <-- 3. *Remove* contents
|           |
|           +-- mock.rs    <-- *Remove* file
|           |
|           +-- tests.rs   <-- *Remove* file
|
+-- Cargo.toml              <-- 4. *Rename* 'pallets/template' to 'pallets/kitties'
```

> You can go ahead and remove `mock.rs` and `tests.rs`. **We won't be learning about
> using these in this tutorial. Have a look at [this how-to guide](/docs/testing/test-transfer) if
> you're curious to learn how testing works.** 


Follow steps 1-3 as depicted in the folder structure above:
1. Rename the `template` folder to `kitties`;
2. Go to the `Cargo.toml` file of the same folder and replace `pallet-template` with `pallet-kitties`. This will be the name of
the pallet as our runtime refers to it.
3. Remove the contents of `lib.rs` and delete `mock.rs` and `tests.rs`.
4. Search are replace `'pallets/template'` to ` 'pallets/kitties'`. This tells our workspace about where to find our Kitties pallet.

We now need to make sure our Kitties node is properly wired up to use the `pallet-kitties` by updating 
`/runtime/Cargo.toml`. 

Search and replace all instances of `pallet-template` to `pallet-kitties` and update the path to the kitties pallet:

```rust
/*--snip--*/
[dependencies.pallet-template]  // <-- 1. Rename to '[dependencies.pallet-kitties]'
default-features = false
path = '../pallets/template'    // <-- 2. Rename to 'path = '../pallets/kitties' '
version = '3.0.0'
/*--snip--*/
[features]
default = ['std']
runtime-benchmarks = [
    /*--snip--*/
    'pallet-template/runtime-benchmarks',   // <-- 3. Rename to 'pallet-kitties/runtime-benchmarks',
    /*--snip--*/
]
std = [
    /*--snip--*/
    'pallet-template/std',  // <-- 4. Rename to 'pallet-kitties/std'
    /*--snip--*/
]
```
Similarily, we need to update our project's `Cargo.toml` file (at the root of our project's directory):

```rust
[profile.release]
panic = 'unwind'

[workspace]
members = [
    'node',
    'pallets/template',   // <-- Replace this with 'pallets/kitties', 
    'runtime',
]
```

:::info
Refer to [this guide](/docs/basics/basic-pallet-integration) to help your learn these basic patterns. You can also read more about pallets in this [knowledgebase article][pallets-kb].  
:::

:::note
All this renaming stuff can get confusing, but it's a good excercise to familiarize yourself with the structure 
of a node template and how things are wired up. To make sure things are clear let's recap what we've done:

- **We've renamed our *project* to `my-substratekitties`.** This only helps keep folders on your local machine organized.
- **We've renamed our *runtime* to `kitties-runtime`.** This is what our node is using to reference the runtime it needs to build &mdash;hence why it's first *named* in `/runtime/Cargo.toml` and then *referenced* as a dependency in `/node/Cargo.toml`. We also renamed how other modules refer to our runtime, which is the same as our runtime's name except with an underscore instead of a hyphen, i.e. `kitties_runtime`.
- **We've renamed our *node* to `kitties-node`.** This is what we'll need to refer to in order to run our chain.
- **We've renamed the `pallets/template` folder to *kitties*.** This only affects the path to link `pallet-kitties` to our runtime in `/runtime/Cargo.toml`. Note that this **not** the name of our pallet &mdash; just the name of the folder where our pallet lives. 
- **We've renamed *pallet-template* to *pallet-kitties*.** This is the name of our pallet as our runtime understands it. 
:::

### 2. Write out the structure for `pallet_kitties`

Now that your node template is ready, we can proceed to start writing our pallet.

[Pallets][pallets-kb] in Substrate are used to define runtime logic. In our case, we'll be creating a single pallet that manages all of the
logic of our Substrate Kitties dApp.
At this point, we're in a good place to lay out the basic structure of our pallet, after which we can check if our node builds without error. By structure, we're talking about outlining the parts inside the `lib.rs` file of our newly created `pallet-kitties`.

Every FRAME pallet has:

- A set of `frame_support` and `frame_system` dependencies.
- Required [attribute macros][macros-kb] (i.e. configuration traits, storage items, hooks and function calls).

:::note
We'll be updating additional dependencies as we progress through the next parts of this tutorial.
:::

In its most bare-bones version, a pallet looks like this:

```rust
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {}
}
```
:::tip Your turn!
Copy over the bare-bones of the pallet into `kitties/src/lib.rs`.

**Hint:** Each part of this tutorial has a file with incomplete code containing comments to guide you to complete it. 
Download the [template code here][template-code] locally and use it to help you progress through each step!
:::

### 3. Implement `pallet_kitties` for your runtime

Now that we have a pallet called `pallet-kitties` we must implement it for our runtime.

- Towards the top of `/runtime/src/lib.rs`, import `pallet_kitties`:

```rust
pub use pallet_kitties;
```

Remove `pub use pallet_template;` &mdash; it no longer exists at this point. 

- Then, include this line after the other trait implementations:

```rust
impl pallet_kitties::Config for Runtime {}
```
Be sure to remove the `impl pallet_template::Config for Runtime` clause entirely as well.

> Since our Kitties pallet doesn't do anything yet, we don't have anything to implement for our runtime! Part II of this series 
> will dive into adding traits to implement for our runtime.

Before we can test whether our node is set-up correctly, let's specify our Kitties pallet inside `construct_runtime!` by simply 
replacing the line corresponding to `pallet_template`: 

```rust
/*--snip--*/
    TemplateModule: pallet_template::{Pallet, Call, Storage, Event<T>}, //<-- Replace this with: 'SubstrateKitties: pallet_kitties::{Pallet, Call, Config<T>, Storage},'
/*--snip--*/
```

Run the following command to build and launch our chain. This can take a little while depending on your machine:

```bash
cargo build --release
```
:::note 
You'll notice the Rust compiler giving you warnings about unused imports. That's fine! Just ignore them &mdash; we're going to 
be using those imports in the later parts of the tutorial.
:::

Assuming everything compiled without error, we can launch our chain and check that it is producing blocks:

```bash
./target/release/kitties-node --tmp --dev
```

Works? If not, make sure you've followed the steps to integrate your new pallet to your runtime. If yes, fantastic!

You don't need to keep your node running. This was just a way to check that your pallet and runtime are
properly configured. In the next steps we will start writing the storage items our Kitty dApp will require.

### 4. Include a storage item to track all Kitties

Let's start by adding the most simple logic we can to our runtime: a function which stores a variable in runtime.

To do this we'll use [`StorageValue`][storagevalue-rustdocs] from Substrate's [storage API][storage-api-rustdocs] which is a trait that depends
on the storage macro.

All that means for our purposes is that for any storage item we want to declare, we must include `#[pallet::storage]` beforehand. Using `StorageValue` as an example, this would look like this:

```rust
#[pallet::storage]
#[pallet::getter(fn get_storage_value)]
pub(super) type SomeStorageValue <T: Config> = StorageValue <
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

**Hint:** follow the same pattern as with the example storage value above, `SomeStorageValue`.
:::

### 5. Build and check your pallet

From the previous step, your pallet should contain a storage item called `AllKittiesCount` which keeps track of a
single `u64` value. As part of the basic setup, we're doing great!

:::info
As mentioned in the [overview of this tutorial series](overview),
you'll be implementing a total of 9 storage items which you'll discover as you
write out your pallet's logic in the next parts.
:::

Before we move on, let's make sure everything compiles. We don't need to rebuild our entire node each time we update our pallet.
Instead, we can use a command that only builds our pallet. From inside your pallet directory, run the following:

```bash
cargo build -p pallet-kitties
```

Does your pallet compile without error? Well done if it does! If not, go back and check that all the macros are in place and that you've included the
FRAME dependencies. 

:::note Congratulations!
You've completed the first part of this series. At this stage, you've learnt the various patterns for:

- Customizing the Substrate node template and including a custom pallet.
- Building a Substrate chain and checking that a target pallet compiles.
- Declaring and using a `u64` storage item.
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
[template-code]: https://github.com/substrate-developer-hub/substrate-how-to-guides/tree/main/static/code/kitties-tutorial
