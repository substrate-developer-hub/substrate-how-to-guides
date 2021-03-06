---
sidebar_position: 1
keywords: parachains
---

# Start a collator node and add more collators

_Learn how to get your chain's collators up and running._

## Goal

Learn how to select collators and set-up collator nodes when launching a parachain.

## Use cases

Launching a parachain.

## Overview

When selecting [collators][collators-polkadot-wiki] for a parachain, it is important to
ensure that there exist _some_ neutral collators to prevent censorship - but not necessarily a majority. It is also important
to avoid having too many collators as they may slow down the network. This guide steps through the considerations to take into
account when launching a collator node.

## Steps

### 1. Collator selection

You are free to choose your method of collator selection. Common methods include stake voting or directly assigning collators via committee or other origins such as democracy. In both cases,
create a pallet to implement the logic that best fits your needs.

#### Stake voting

See the [`collator-selection` pallet][stake-voting-pallet] for a practical example on implementing stake voting to select collators.

#### Using on-chain governance

Implement a special origin that allows members of that origin to
become a collator. Use the democracy pallet to elect these members and define them in your pallet dedicated to handling collartor selection:

```rust
    /// Configuration trait of this pallet.
	#[pallet::config]
	pub trait Config: frame_system::Config {
        // --snip-- //
        type MySpecialOrigin: EnsureOrigin<Self::Origin>;
    }
    // --snip-- //
    #[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Some set-collator dispatchable.
		#[pallet::weight(some_weight)]
		pub fn set_collator( origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            T::MySpecialOrigin::ensure_origin(origin)?;
            // --snip-- //
        }
```

:::note
There are also different ways to implement incentives for collators. Take a look at [this example](https://github.com/PureStake/moonbeam/blob/master/pallets/parachain-staking/src/lib.rs) to explore how.
:::

### 2. Starting a collator node

Refer to [this guide](https://substrate.dev/cumulus-workshop/#/en/3-parachains/1-launch?id=start-the-collator-node) to start and set up a collator node.

### 3. Adding collators

Refer to [this instruction](https://substrate.dev/cumulus-workshop/#/en/3-parachains/4-more-nodes?id=start-the-second-collator) to add more collators.

## Examples

- [Cumulus implementation of collator selection](https://github.com/paritytech/cumulus/blob/master/pallets/collator-selection/src/lib.rs) with incentives using transaction fees
- [Moonbeam implementation of collator selection](https://github.com/PureStake/moonbeam/blob/master/pallets/parachain-staking/src/lib.rs) using an inflationary monetary policy staking scheme.

## Resources

- [Parachain DevOps best practices](https://gist.github.com/lovelaced/cddc1c7234b883ee37e71cf4a1d63cac)
- [DevOps for parachains office hour](https://drive.google.com/file/d/1-nQ_SI2XK6vxPQvORWuv68Yj0UDz5FrO/view)

[collators-polkadot-wiki]: https://wiki.polkadot.network/docs/learn-collator
[stake-voting-pallet]: https://github.com/paritytech/cumulus/blob/master/pallets/collator-selection/src/lib.rs
