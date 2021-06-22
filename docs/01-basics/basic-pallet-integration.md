---
sidebar_position: 1
keywords: basics, beginner, runtime
---

# Basic pallet integration

_"I haven't quite yet internalized the basic process of integrating a pallet in my runtime...Ugh!" Totally get it, it's OK, you won't even need this way sooner than you know._

[![Try on playground](https://img.shields.io/badge/Playground-Node_Template-brightgreen?logo=Parity%20Substrate)](https://playground.substrate.dev/?deploy=node-template)

## Goal

Learn the steps for integrating a pallet to your runtime.

## Use cases

Including a pallet that implements an event and a call for runtime.

## Overview

This guide is an extension to the ["Add a Pallet to Your Runtime" tutorial][add-a-pallet-tutorial], intended as a resource 
for developers new to Substrate looking to quickly integrate a pallet to their runtime.

## Steps

### 1. Import your pallet

Assuming a pallet called `pallet_something` is created, the first step is to import it in `runtime/lib.rs`:

```rust
// Import your pallet.
pub use pallet_something;
```

### 2. Include it in your runtime 

Now, configure its runtime implementation. Our pallet only has `Event` and `Call` types exposed to the runtime:

```rust
// Configure your pallet.
impl pallet_something::Config for Runtime {
	type Event = Event;
	type Call = Call;
}
```

Then, declare your pallet and the items it exposes in `construct_runtime!`, including the additional `Module` and `Storage` 
types for the runtime macro: 

```rust
construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
/* --snip-- */
		Something: pallet_something::{Module, Call, Storage, Event<T>}, 
/* --snip-- */
	}
);
```

### 3. Update `Cargo.toml`

In `/runtime/Cargo.toml`, include your pallet as a local dependency and include it in `std`:

```rust
/* --snip-- */
# local dependencies
pallet-something = { path = '../pallets/pallet-something', default-features = false, version = '3.0.0' }
/* --snip-- */
std = [
'pallet-something/std',
/* --snip-- */
]
```

## Examples

- [Template pallet](https://github.com/substrate-developer-hub/substrate-node-template/blob/master/pallets/template/src/lib.rs#L1-L107)
- [Substrate node template][playground-gov]

## Related material

- [Mock runtime][mock-runtime] 

[add-a-pallet-tutorial]: https://substrate.dev/docs/en/tutorials/add-a-pallet/import-a-pallet
[playground-gov]: playground.substrate.dev
[mock-runtime]: https://substrate.dev/docs/en/knowledgebase/runtime/tests#mock-runtime-environment
