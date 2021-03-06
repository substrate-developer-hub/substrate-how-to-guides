---
sidebar_position: 2
keywords: parachains
---

# Set-up your runtime and client

_A series of steps you're going to want to take before deploying your runtime as a parachain._

## Goal

- Ensure runtime weights are corret.
- Correctly deploy a runtime.

## Use cases

Launching a parachain.

## Overview

When launching a parachain, it is important to make sure a chain's runtime is properly setup. This includes benchmarking prior to storage migrations, checking that weights are correctly implemented and making sure the ProtocolID IS unique.

## Steps

### 1. Set a unique ProtocolID

In order to set a unique ProtocolID, change make sure you use some nonce or salt value. This is set in
`/client/network/src/config`:

```rust
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ProtocolId(smallvec::SmallVec<[u8; 6]>);

impl<'a> From<&'a str> for ProtocolId {
	fn from(bytes: &'a str) -> ProtocolId {
		ProtocolId(bytes.as_bytes().into())
	}
}
```

This step is important &mdash; you wouldn't want to connect to the wrong network!

:::tip
Refer to [this guide](/docs/pallet-design/randomness) for some ideas on creating a unique value to use here.
:::

:::note Memory Profiling
[Profiling your collator][profiling-kb] should be done to analyze memory leaks,
identify where memory consumption is happening, define temporary
allocations, and investigate excessive memory fragmentation within
applications.
:::

### 2. Check runtime weights

Use benchmarking to verify that your runtime weights are correct.

:::note
Refer to this [knowledgebase article][benchmarking-kb] on
benchmarking for additional information.
:::

#### Customize weights

Make sure that each pallet in your runtime employs the correct weighting system. Default Substrate weight **are not** to be used in production, as a general rule.

#### Set block weight limit

It is recommended to have a block weight limit (block production time) of 0,5 seconds in the beginning due to uncertainties in block execution time. As the execution time of the network stabilizes the weight limit can be increased to 2 seconds.

### 3. Runtime deployment

#### Minimize the size of your runtime

Generally, when launching a parachain, it is important to use the **compressed version of the runtime** to lower the amount of data being transferred.

- It is recommended to launch a parachain with limited functionality and gradually increase it with runtime upgrades. The reason behind that is that during a runtime upgrade both the previous runtime and the new runtime are included in the PoVBlock and therefore if the changes are large enough the block might be rejected by the Relay Chain due to PoVBlock size limits.

- If the runtime is included in the state proof, ensure the PoV block (i.e. the set of extrinsics, including the new runtime, the PoV state proof, potentially the old runtime) fits within the PoVBlock size limit. If the runtime is not included in the state proof, the size limit of the new runtime will be much higher.

:::tip
You can check the maximum PoVBlock size [here](https://github.com/paritytech/polkadot/blob/a620156c0cdb46991b8eae89b99d1941aa8d9e18/primitives/src/v1/mod.rs#L206) or in the Polkadot-JS Apps UI: _Developers_ -> _ParachainsConfiguration_ -> _ActiveConfiguration_)
:::

:::info
Here you can see an example of how to [limit](https://github.com/paritytech/cumulus/blob/59cdbb6a56b1c49009413d66ba2232494563b57c/polkadot-parachains/statemine/src/lib.rs#L148) and [enable](https://github.com/paritytech/cumulus/pull/476/files#diff-09b95657e9aa1b646722afa7944a00ddc2541e8753254a86180b338d3376f93eL151) functionality with filters as implemented in [Statemint][statemint].
:::

#### For large runtimes

It is less favorable to perform storage upgrades for large runtimes. In these cases, you can:

1. Generate the genesis state of your chain with full runtime functionality (including all the pallets)

2. Remove all pallets that you will not need upon parachain launch from your runtime

3. Re-build the WASM blob (validation logic) and the runtime of the chain

4. Register your parachain with the updated genesis and the WASM blob generated in (3)

5. After your parachain is live you can upgrade your runtime on-chain to include the missing pallets (ensure that pallet indices and names match those used to generate the genesis state in step (1) without having to do storage migrations. For more information on on-chain runtime upgrades refer to the next section.

## Examples

- [Statemine runtime deployment](https://github.com/paritytech/cumulus/pull/476)

## Resources

#### Knowledgebase

- [Benchmarking][benchmarking-kb]

[benchmarking-kb]: https://substrate.dev/docs/en/knowledgebase/runtime/benchmarking
[profiling-kb]: https://substrate.dev/docs/en/knowledgebase/integrate/memory-profiling
[statemint]: https://github.com/paritytech/cumulus/tree/master/polkadot-parachains/statemint
