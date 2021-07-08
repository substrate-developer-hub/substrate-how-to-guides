---
sidebar_position: 7
keywords: parachains
---

# How to register your parachain
_ ... _

## Goal

## Use cases
Launching a parachain.

## Overview



## Steps

To register your parachain you need to provide your ParaID, genesis state and your compressed WASM validation logic.
 - Generate [parachain Genesis state](https://substrate.dev/cumulus-workshop/#/en/3-parachains/1-launch?id=generate-parachain-genesis-state)
- [Obtain WASM runtime validation logic](https://substrate.dev/cumulus-workshop/#/en/3-parachains/1-launch?id=obtain-wasm-runtime-validation-function). Bear in mind that maximum size of the WASM blob is 750 kilobytes on Kusama 
- [Register the parachain](https://substrate.dev/cumulus-workshop/#/en/3-parachains/2-register?id=parachain-registration). This transaction requires a deposit. The amount of the deposit depends on the size of the WASM blob and the genesis states. Here you can find exact formulas for deposit calculation for [Kusama](https://github.com/paritytech/polkadot/blob/04b2383ba6685bacc63a2eb4a1949aebadbc624b/runtime/kusama/src/constants.rs#L26) and [Polkadot](https://github.com/paritytech/polkadot/blob/04b2383ba6685bacc63a2eb4a1949aebadbc624b/runtime/polkadot/src/constants.rs#L27)


## Examples

## Resources
#### Rust docs
#### Knowledgebase 
