---
sidebar_position: 6
keywords: [pallet design, intermediate, runtime]
---

# Use the Contracts pallet

_Create the basis for building Wasm smart contracts using FRAME._

## Goal

Add the Contracts pallet to your runtime to be able to use Wasm smart contracts in your blockchain.

## Use cases

- Smart Contract develoment
- On-chain execution of Wasm binaries

## Overview

This guide will show you how you can add the [Contracts pallet][contracts-frame] to your runtime in order to allow your blockchain
to support Wasm smart contracts. You can follow similar patterns to add additional FRAME pallets to your runtime,
however you should note that each pallet is a little different in terms of the specific configuration settings
needed to use it correctly.

:::note
You should already have the latest version of the Substrate Node Template compiled on your computer to follow this guide.
If you haven't already done so, refer to [this tutorial][create-first-chain-tutorial].
:::

## Steps

### 1. Import the dependencies

Refer to this guide to properly include Contracts in your runtime.

This includes **updating `runtime/Cargo.toml`** and **runtime/Cargo.toml` with:** - `pallet-contracts` - `pallet-contracts-primitives`

### 2. Add the Contracts pallet to your runtime

Now you'll have to implement the Contract's pallet [configuration traits][contracts-config-rustdocs] in order for your runtime to use it properly.

#### Implement `pallet_contracts`

Start by making sure you've included all of the types that `pallet_contracts` exposes. You can copy these from [FRAME's source code][contracts-frame] (assuming versioning is equivalent to the imported crate). Here's what you need to add inside `runtime/lib.rs` &mdash; only the first 4 types are shown:

```rust
impl pallet_contracts::Config for Runtime {
    type Time = Timestamp;
    type Randomness = RandomnessCollectiveFlip;
    type Currency = Balances;
    type Event = Event;
    /* --snip-- */
```

#### Parameter types

Some of these types require `parameter_types`. Have a look at their implementation in [this][bin-runtime-contracts-frame] runtime to make sure you include everything. We'll take `DeletionQueueDepth` as one example. Parameter types go right above `impl pallet_contracts::Config for Runtime` :

```rust
parameter_types! {
     /* --snip-- */
	pub DeletionQueueDepth: u32 = ((DeletionWeightLimit::get() / (
			<Runtime as pallet_contracts::Config>::WeightInfo::on_initialize_per_queue_item(1) -
			<Runtime as pallet_contracts::Config>::WeightInfo::on_initialize_per_queue_item(0)
		)) / 5) as u32;
        /* --snip-- */
    }
```

Notice how the above parameter type requires `WeightInfo`. This requires you to add the following to the top of `runtime/lib.rs`:

```rust
use pallet_contracts::weights::WeightInfo;
```

Similarly, other parameter types use constants such as `DAYS`, `MILLICENTS` and `AVERAGE_ON_INITIALIZE_RATIO`.

Define these towards the top of your `runtime/lib.rs` file where the other constants exists:

```rust
// Contracts price units.
pub const MILLICENTS: Balance = 1_000_000_000;
pub const CENTS: Balance = 1_000 * MILLICENTS;
pub const DOLLARS: Balance = 100 * CENTS;

const fn deposit(items: u32, bytes: u32) -> Balance {
    items as Balance * 15 * CENTS + (bytes as Balance) * 6 * CENTS
}

/// We assume that ~10% of the block weight is consumed by `on_initalize` handlers.
/// This is used to limit the maximal weight of a single extrinsic.
const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(10);
```

#### Add an instance in runtime for `pallet_contracts`

Ceate an instance of the Contracts pallet in `construct_macro!` inside `runtime/lib.rs`:

```rust
/* --snip-- */
 Contracts: pallet_contracts::{Module, Call, Config<T>, Storage, Event<T>},
 /* --snip-- */
```

### 3. Add API dependencies

:::info
Some pallets, including the Contracts pallet, expose custom runtime APIs and RPC endpoints. In the case of the Contracts pallet, this enables reading contracts state from off chain.
:::

In this guide, we want to use the Contracts pallet to make calls to our node's storage without making a transaction.

To achieve this, we'll use another pallet called `pallet-contracts-rpc-runtime-api`.

#### Import dependencies

Just like in the first step of this guide, update `Cargo.toml` to add `pallet-contracts-rpc-runtime-api`.

Now we can add the [`ContractsApi`][contracts-api-rustdocs] dependency required to implement the Contracts runtime API.

Add this with the other `use` statements.

#### Implement the Contracts runtime API

We're now ready to implement the contracts
runtime API.

This happens in the
`impl_runtime_apis! `macro near the end of your
runtime.

Make sure to add the following functions that the `ContractsApi` exposes:

- **`call()`**: returns `pallet_contracts_primitives::ContractExecResult { Contracts::bare_call(origin, dest, value, gas_limit, input_data)}`
- **`get_storage()`**: returns `pallet_contracts_primitives::GetStorageResult {Contracts::get_storage(address, key)}`
- **`rent_projection()`**: returns `pallet_contracts_primitives::RentProjectionResult<BlockNumber> {Contracts::rent_projection(address)}`

#### Add RPC API extension

To be able to call the runtime API, we must add the RPC to the node's service.

In `node/Cargo.toml`, add the dendencies for `pallet-contracts` and `pallet-contracts-rpc`.

:::note Unsure what version to include?

Use the latest version as indicated on [crates.io][pallet-crates].
:::

This RPC does not contain access to the Contracts pallet by default. To interact with this pallet, we have to **extend the existing RPC and add the Contracts pallet** along with its API.

In `node/src/rpc.rs`, add this line to the `where` clause in `create_full<C, P>`:

```rust
 C::Api: pallet_contracts_rpc::ContractsRuntimeApi<Block, AccountId, Balance, BlockNumber>,
```

And add the contracts RPC API extension using:

```rust
 // Contracts RPC API extension
   io.extend_with(
       ContractsApi::to_delegate(Contracts::new(client.clone()))
   );
```

## Examples

- `canvas-node` [runtime](https://github.com/paritytech/canvas-node/blob/master/runtime/src/lib.rs#L361)
- `canvas-node` [rpc file](https://github.com/paritytech/canvas-node/blob/master/node/src/rpc.rs)

## Related material

#### Rust docs

- [`pallet_contracts` crate](https://substrate.dev/rustdocs/latest/pallet_contracts/index.html)
- [`pallet_contracts_rpc` crate](https://substrate.dev/rustdocs/latest/pallet_contracts_rpc/index.html)

[contracts-frame]: https://substrate.dev/rustdocs/latest/pallet_contracts/
[create-first-chain-tutorial]: https://substrate.dev/docs/en/tutorials/create-your-first-substrate-chain/
[contracts-config-rustdocs]: https://substrate.dev/rustdocs/latest/pallet_contracts/trait.Config.html
[contracts-frame]: https://github.com/paritytech/substrate/blob/master/frame/contracts/src/lib.rs#L144
[bin-runtime-contracts-frame]: https://github.com/paritytech/substrate/blob/master/bin/node/runtime/src/lib.rs#L786
[contracts-api-rustdocs]: https://substrate.dev/rustdocs/latest/pallet_contracts_rpc_runtime_api/trait.ContractsApi.html
[pallet-crates]: https://crates.io/search?q=pallet-contracts-rpc
