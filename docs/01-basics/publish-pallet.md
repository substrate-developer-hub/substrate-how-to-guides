---
sidebar_position: 8
keywords: runtime, beginner, basics
---

# Publish a pallet to its own crate

_Make your pallet available for others to use!_

## Goal

- Learn the patterns to write a Substrate pallet that lives in its own crate
- Publish a pallet to Github and Crates.io
- Include your pallet in a Substrate Node from Github and Crates.io

## Use cases

Make your pallet available for other developers to use remotely.

## Overview

Writing a Substrate pallet in its own Rust crate, and publishing it allows other
blockchain developers to easily use your pallet in their runtime by simply including those same
four lines of code in their runtime's `Cargo.toml` files and updating their runtime's `lib.rs` file.

This guide is not going to go through writing a pallet directly as part of the node template, but rather as a separate
Rust crate. This approach allows us to publish our pallet separately from our node and also allows
others to easily import this pallet into their own Substrate runtime.

## Steps

### 1. Renaming your crate

In the `Cargo.toml` file, you must update the crate's name. Update the value
of the `package.name` attribute in the `Cargo.toml` file to `test-pallet`.

The `package` section of the `Cargo.toml` file now looks like:

**`pallets/test-pallet/Cargo.toml`**

```toml
[package]
authors = ['Substrate DevHub <https://github.com/substrate-developer-hub>']
description = 'FRAME pallet template'
edition = '2018'
homepage = 'https://substrate.dev'
license = 'Unlicensed'
name = 'test-pallet'
repository = 'https://github.com/substrate-developer-hub/substrate-pallet-template/'
version = '3.0.0'
```

#### Compile the Template Pallet

You should be able to successfully check the Substrate pallet template with:

```bash
cd test-pallet
SKIP_WASM_BUILD=1 cargo check
```

:::note Your Pallet's `std` Feature

In your `pallets/test-pallet/Cargo.toml` file, you will notice a few lines about the "`std`
feature". In Rust, when you enable `std`, you give your project access to
[the Rust standard libraries](https://doc.rust-lang.org/std/). This works just fine when building
native binaries.
However, Substrate also builds the runtime code to WebAssembly (Wasm). In this case we use cargo
features to disable the Rust standard library. Thus, all the dependencies we use for our pallet, and
our entire runtime, must be able to compile with
[`no_std`](https://rust-embedded.github.io/book/intro/no-std.html) feature. Our `Cargo.toml` file
tells our pallet's dependencies to only use their `std` feature when this pallet also uses its `std` feature.
:::

### 2. Ensure consistent dependencies

#### FRAME dependencies

All Substrate pallets will depend on some low-level FRAME libraries such as `frame-system` and
`frame-support`. These libraries are pulled from crates.io. When people build their own FRAME-based
runtimes, they will also have dependencies on these low-level libraries. You will need to ensure
consistent dependencies between your pallet and your runtime.

**`pallets/test-pallet/Cargo.toml`**

```TOML
# --snip--
[dependencies]
frame-support = { default-features = false, version = '3.0.0' }
# --snip--
```

From the above snippet, we see that this pallet template depends on version `3.0.0` of the low-level
libraries. Thus it can be used in runtimes that also depend on `3.0.0`.

:::note
Note that substrate adheres to the [semver](https://semver.org/) standards - thus each release is
on the form `major.minor.patch`. In general it is _not_ expected that major releases are compatible!
Thus if you are developing a pallet, or integrating ones, be sure to match versions to keep things
all working correctly.
:::

#### Dev dependencies

The final section of the `Cargo.toml` file specifies the dev dependencies. These are the
dependencies that are needed in your pallet's tests, but not the actual pallet itself.

**`pallets/test-pallet/Cargo.toml`**

```TOML
# --snip--
[dev-dependencies]
sp-core = { default-features = false, version = '3.0.0' }
# --snip--
```

You can confirm that the tests in the Substrate pallet template pass with:

```bash
SKIP_WASM_BUILD=1 cargo test
```

When updating this pallet to include your own custom logic, you will likely add dependencies of your
own to this `Cargo.toml` file.

### 3. Add your pallet to your node

Refer to [this guide](/docs/basics/basic-pallet-integration) on how to complete this step.

### 4. Run your node

At this point you have the pallet packaged up in its own crate and included in your node's runtime.

Make sure you're back in the node template's root directory, then compile the node and start in
development mode with the following command:

```bash
cargo build --release
./target/release/node-template --tmp --dev
```

Now, start the
[Polkadot-JS Apps connecting to your local node](https://polkadot.js.org/apps/#/extrinsics?rpc=ws://127.0.0.1:9944)
to confirm that the pallet is working as expected.

:::note
You can also manually set the node URL in Polkadot-JS Apps by navigating to the **Settings** tab, and have the **remote node/endpoint to connect to** set to **Local Node**.
:::

### 5. Publish your pallet

Once your pallet is no longer in test phase, you should consider publishing it to [GitHub](https://github.com/) or
[crates.io](https://crates.io/), the crate registry for the Rust community.

#### Publishing on GitHub

To publish on GitHub, you need to
[create a GitHub repository](https://help.github.com/en/articles/create-a-repo) and
[push your pallet's code](https://help.github.com/en/articles/pushing-to-a-remote) to it.

#### Publishing on Crates.io

Crates.io allows permissionless publishing. Learn the procedure following their own guide about
[publishing on crates.io](https://doc.rust-lang.org/cargo/reference/publishing.html)

### 6. Updating your runtime's dependencies

With your pallet now published on GitHub, crates.io, or both, we can update your runtime to use the
code that is published instead of a hard-coded file system path.

#### Dependencies from GitHub

**`runtime/Cargo.toml`**

```TOML
[dependencies.your-pallet-name]
default_features = false
git = 'https://github.com/your-username/your-pallet'
branch = 'master'

# You may choose a specific commit or tag instead of branch
# rev = '<git-commit>'
# tag = '<some tag>
```

#### Dependencies from Crates.io

**`runtime/Cargo.toml`**

```TOML
[dependencies.your-pallet-name]
default_features = false
version = 'some-compatible-version'
```

Compile one more time and notice that Cargo now grabs your pallet from GitHub or crates.io instead of using the local files.

## Examples

- [Example pallet](https://github.com/paritytech/substrate/tree/master/frame/example)

## Resources

#### Other

- [The Cargo book](https://doc.rust-lang.org/stable/cargo/)
- More about [Rust and WebAssembly](https://rustwasm.github.io/)
