---
sidebar_position: 3
keywords: weights, runtime, FRAME v1
---

# Quadratic weighting struct

_A tool for weighting more complex transactions._

## Goal

Calculate transaction weights for transactions with 3 or more parameters.

## Use cases

Calculate correct weight based on data within a function, required to calculate transaction fees.

## Overview

This is a more complex way scale to weight transactions. It computes weight according to the following formula:

_a*x^2 + b*y + c_

Where a, b, and c are fields in the struct, and x and y are transaction parameters. Have a look at the [examples][#examples]
section to see it implemented and used in context.

## Steps

### 1. Write the `Quadratic` struct

Write a weighting struct that weighs transactions where the first parameter is a boolean value.

```rust
pub struct Quadratic(u32, u32, u32);

impl WeighData<(&u32, &u32)> for Quadratic {
	fn weigh_data(&self, (x, y): (&u32, &u32)) -> Weight {
		let ax2 = x.saturating_mul(*x).saturating_mul(self.0);
		let by = y.saturating_mul(self.1);
		let c = self.2;

		ax2.saturating_add(by).saturating_add(c).into()
	}
}
```

### 2. Classify dispatch calls

Since this implementation of `WeighData` requires a `DispatchClass`, use [`default`][dispatchclass-rustdocs] to classify all calls as normal.

```rust
// Implement ClassifyDispatch.
impl<T> ClassifyDispatch<T> for Quadratic {
	fn classify_dispatch(&self, _: T) -> DispatchClass {
		// Classify all calls as Normal (which is the default)
		Default::default()
	}
}
```

### 3. Implement `PaysFee`

Last, specify how `PaysFee` is used for the custom `WeighData` struct.

```rust
// Implement PaysFee.
impl<T> PaysFee<T> for Quadratic {
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

[dispatchclass-rustdocs]: https://substrate.dev/rustdocs/v3.0.0/frame_support/weights/enum.DispatchClass.html
