---
sidebar_position: 2
keywords: weights, runtime, FRAME v1
---

# Linear weighting struct

_Get the simple things down first._

## Goal

Understand how to calculate transaction weights using a custom weighting struct for single transaction values.

## Use cases

Calculate correct weight for a transaction involving a `u32`.

## Overview

This guide goes over the components of a simple weighting struct designed for a single argument dispatch of type `u32`.
The ultimate weight of the transaction is the product of the transaction parameter and the field of this struct.

## Steps

### 1. Write the `WeighData` struct

Using [`WeighData`][impl-weighdata-rustdocs], write a weighting struct that takes a single `u32` parameter:

```rust
pub struct Linear(u32);

impl WeighData<(&u32,)> for Linear {
	fn weigh_data(&self, (x,): (&u32,)) -> Weight {
		// Use saturation so that an extremely large
		// parameter value does not cause overflow
		x.saturating_mul(self.0).into()
	}
}
```

### 2. Classify dispatch calls

Since this implementation of `WeighData` requires a `Dispatch`, use [`default`][dispatchclass-rustdocs] to classify all calls as normal&mdash;as opposed to operational.

```rust
// Implement ClassifyDispatch
impl<T> ClassifyDispatch<T> for Linear {
	fn classify_dispatch(&self, _: T) -> DispatchClass {
		// Classify all calls as Normal (which is the default).
		Default::default()
	}
}
```

### 3. Implement `PaysFee`

Implement the [`PaysFee`][paysfee-rustdocs] trait to indicate whether fees should actually be charged from the caller. If not, the weights are still applied toward the block maximums.

```rust
// Implement PaysFee
impl<T> PaysFee<T> for Linear {
	fn pays_fee(&self, _: T) -> Pays {
		Pays::Yes
	}
}
```

## Examples

- Feeless transaction pallet
- pallet-weights

## Related material

#### How-to guides

- [Linear weighting struct](./linear-weight-struct)
- [Quadratic weighting struct](../300/quadratic-weight-struct)

#### Knowledgebase

- [Transaction Weights](https://substrate.dev/docs/en/knowledgebase/learn-substrate/weight)
- [Transaction Fees](https://substrate.dev/docs/en/knowledgebase/runtime/fees)

#### Other

- [Transaction fees in Polkadot](https://wiki.polkadot.network/docs/en/learn-transaction-fees)

[impl-weighdata-rustdocs]: https://substrate.dev/rustdocs/v3.0.0/frame_support/weights/trait.WeighData.html#impl-WeighData%3CT%3E-for-(Weight%2C%20DispatchClass%2C%20Pays
[paysfee-rustdocs]: https://substrate.dev/rustdocs/v3.0.0/frame_support/weights/trait.PaysFee.html
[classifydispatch-rustdocs]: https://substrate.dev/rustdocs/v3.0.0/frame_support/weights/trait.ClassifyDispatch.html
