---
sidebar_position: 4
keywords: storage migration, testing, runtime
---

# Include and use `try-runtime` in your runtime

## Goal
Pinpoint some runtime state and scrape it, put it into `TestExternalities` and test it.

## Use cases


## Overview


## Steps

### 1. Adding `runtime` dependencies

#### In `runtime/Cargo.toml`

Add FRAME dependency:

```rust
frame-try-runtime = { version = "0.9.0", default-features = false, path = "../../../frame/try-runtime", optional = true }
    /* --snip-- */
    std = [
    /* --snip-- */
    "frame-try-runtime/std",
]
```

#### In `runtime/Cargo.toml`, for every pallet in your runtime:

```rust
try-runtime = [
	"frame-executive/try-runtime",
	"frame-try-runtime",
	"frame-system/try-runtime",
]
```

#### In `runtime/src/lib.rs`, implement it for your Runtime:

```rust
    #[cfg(feature = "try-runtime")]
	impl frame_try_runtime::TryRuntime<Block> for Runtime {
		fn on_runtime_upgrade() -> Result<(Weight, Weight), sp_runtime::RuntimeString> {
			log::info!("try-runtime::on_runtime_upgrade.");
			let weight = Executive::try_runtime_upgrade()?;
			Ok((weight, BlockWeights::get().max_block))
		}
	}
```

### 2. Adding `node` dependencies

#### In `node/Cargo.toml`

```rust
[features]
/* --snip-- */
try-runtime = ['node-template-runtime/try-runtime']

/* --snip-- */
[dependencies.frame-try-runtime]
default-features = false
version = "0.9.0"
git = 'https://github.com/paritytech/substrate.git'
optional = true

/* --snip-- */
[dependencies.try-runtime-cli]
 git = 'https://github.com/paritytech/substrate/master'
 optional = true 
/* --snip-- */
cli = [
    /* --snip-- */
    "try-runtime-cli",
    ]
cli = [
	/* --snip-- */
	"try-runtime-cli",
]
/* --snip-- */
try-runtime = [
	"node-runtime/try-runtime",
	"try-runtime-cli",
]

/* --snip-- */
```

#### In `node/src/cli.rs` add the subcommands

```rust
/* --snip-- */
    /// Try some command against runtime state.
	#[cfg(feature = "try-runtime")]
	TryRuntime(try_runtime_cli::TryRuntimeCmd),

	/// Try some command against runtime state. Note: `try-runtime` feature must be enabled.
	#[cfg(not(feature = "try-runtime"))]
	TryRuntime,
/* --snip-- */
```

#### In `node/src/commands.rs`, add:

```rust
/* --snip-- */
        #[cfg(feature = "try-runtime")]
		Some(Subcommand::TryRuntime(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				// we don't need any of the components of new_partial, just a runtime, or a task
				// manager to do `async_run`.
				let registry = config.prometheus_config.as_ref().map(|cfg| &cfg.registry);
				let task_manager = sc_service::TaskManager::new(
					config.task_executor.clone(),
					registry,
				).map_err(|e| sc_cli::Error::Service(sc_service::Error::Prometheus(e)))?;

				Ok((cmd.run::<Block, Executor>(config), task_manager))
			})
		},

		#[cfg(not(feature = "try-runtime"))]
		Some(Subcommand::TryRuntime) => {
			Err("TryRuntime wasn't enabled when building the node. \
				You can enable it with `--features try-runtime`.".into())
		},
/* --snip-- */
```

:::note
If you're using custom pallets in your workspace, make sure you included 
`try-runtime` in the dependencies inside the `pallets/pallet_name/Cargo.toml` file of your workspace.
:::

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

### Write remote tests
Just like writing unit tests, to use `try-runtime` create an externalities instance and call `execute_with` on it. 

## Examples

## Resources
#### How-to guides

#### Other

[tryruntime-api-rustdocs]: https://crates.parity.io/frame_try_runtime/trait.TryRuntime.html



