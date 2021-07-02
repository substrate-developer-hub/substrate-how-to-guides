---
sidebar_position: 4
keywords: storage migration, testing, runtime
---

# Test a storage migration with `try-runtime`

## Goal

## Use cases

## Overview

The initial motivation came from the need to extend the capabilities of `test_externalities` such that it can populate its database with real node data.

try-runtime uses `remote_externalities` instead. Put simply, this allows developers to:

- Connect to a remote node.
- Take the data for that node.
- Write tests for that node.

It combines: Runtime, Calling into some runtimeAPI and querying the State of a live chain.

With externalities populated with a specified chain state, this allows developers to call into a given runtime API and test on the state of one or many pallets in a chosen block.

The most common use case for `try-runtime` is with storage migrations and runtime upgrades. With storage migrations being the potentially more complex type of a runtime upgrade, this article will cover what makes this tool especially useful for these such scenarios. The goal is to pinpoint one state and scrape it, put it into `TestExternalities` and test it.

### TestExternalities

(use diagram)
Externalities provides storage to the runtime
There’s an externalities implementation called TestExternalities
Using RPCs we can just get the K,V pairs for the hashmap of storage that TestExternalities populates for us
The runtime can’t store state. It uses externalities to store it.

### Using try-runtime features

Using features to call into hooks from `OnRuntimeUpgrade`

See how `try_runtime_upgrade` calls into the hook of the pallet.

```rust
try_runtime_upgrade()

pre_upgrade() and post_upgrade()
```

Command:

```bash
cargo run -- --release --features=try-runtime try-runtime on-runtime-upgrade live ws://localhost:9944
```

### Importing try-runtime into your project

1. Update imports:

In `node/runtime/Cargo.toml`:

Add frame dependency

```rust
frame-try-runtime = { version = "0.9.0", default-features = false, path = "../../../frame/try-runtime", optional = true }
// -snip
std = [
    // -snip
    "frame-try-runtime/std",
]
```

And also [here](https://github.com/paritytech/substrate/blob/cf4e320398cf3b8ef8a2c240d7495c25b3b73b08/bin/node/runtime/Cargo.toml#L210-L250).

In `node/Cargo.toml`:

```rust
cli = [
	//- snip
	"try-runtime-cli",
]
// -- snip
# Enable features that allow the runtime to be tried and debugged. Name might be subject to change
# in the near future.
try-runtime = [
	"node-runtime/try-runtime",
	"try-runtime-cli",
]
```

### How to import it in a custom chain:

Add it to the cli commands of your chain
Implement runtime API in node/runtime/src/:

```rust
#[cfg(feature = “try-runtime”)]
    impl frame_try_runtime::TryRRuntime<Block> for Runtime {
    fn on_runtime_upgrade() -> Result<(Weight, Weight),         sp_runtime::RuntimeString> {
        let weight = Executive::try_runtime_upgrade()?;
        Ok((weight, RuntimeBlockWeights::get().max_block))
        }
    }
```

## Steps

### 1. Write mock runtime

## Examples

## Resources

#### How-to guides

#### Other

[tryruntime-api-rustdocs]: https://crates.parity.io/frame_try_runtime/trait.TryRuntime.html
