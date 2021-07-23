---
sidebar_position: 3
theme: codeview
code: code/ringbuffer-pseudo.rs
keywords:
  - storage migration
  - runtime
---

# Ringbuffer queue

_For those dealing with a little out of the ordinary data structures._

## Goal

Build a transient storage adapter for a FIFO (First-in-first-out) queue.

## Use cases

Handling complex data structures stored in storage.

## Overview

A [ringbuffer][ringbuffer-wikipedia] that abstracts over storage can be a useful tool when handling storage migrations for more sophisticated pallets. This guide is intended to step you through how to build a storage adapter and use it for a [FIFO][fifo-wikipedia] queue. It will guide you through building a function to overwrite old storage values within pre-defined bounds.

## Steps

### 1. Defining the RingBuffer trait

The `RingBuffer` trait will serve as the interface to our queue. It must define:

- `commit`: to sync the changes made to the underlying storage.
- `push`: to push an item onto the end of the queue
- `pop`: to pop an item from the start of a queue
- `is_empty`: checks if queue is empty

Define it as shown below:

```rust
pub trait RingBufferTrait<Item> where Item: Codec + EncodeLike
{
	/// Store all changes made in the underlying storage.
	fn commit(&self);
	/// Push an item onto the end of the queue.
	fn push(&mut self, i: Item);
	/// Pop an item from the start of the queue.
	fn pop(&mut self) -> Option<Item>;
	/// Return whether the queue is empty.
	fn is_empty(&self) -> bool;
}
```

### 2. Specifying the Ringbuffer transient

#### Start and End bounds

We will be storing the start and end of the
ringbuffer separately from the actual items and will thus need to store these in our struct:

```rust
pub struct RingBufferTransient<Index>
where
	Index: Codec + EncodeLike + Eq + Copy,
{
	start: Index,
	end: Index,
}
```

#### Defining storage interface bounds

In order to access the underlying storage we need to include the bounds in storage that can be accessed.

Let type `B` correspond to the specified bounds; `M` to the item storage; and `Item` to specify the constraints on `M`. Write this as follows:

```rust
pub struct RingBufferTransient<Item, B, M, Index>
where
	Item: Codec + EncodeLike,
	// A StorageValue storing a tuple of indices (Index, Index)
	B: StorageValue<(Index, Index), Query = (Index, Index)>,
	// A StorageMap mapping from our Index type to the Item type
	M: StorageMap<Index, Item, Query = Item>,
	//
	Index: Codec + EncodeLike + Eq + Copy,
{
	start: Index,
	end: Index,
	_phantom: PhantomData<(Item, B, M)>,
}
```

:::note
The `Query` type is specified to help with type inference (because the value returned can
be different from the stored representation).
The [`Codec`][codec-rustdocs] and [`EncodeLike`][encodelike-rustdocs]
type constraints make sure that both items and indices can be stored in storage.
The [`PhantomData`][phantomdata-rustdocs] is needed in order
to "hold on to" the types during the lifetime of the transient object.
:::

#### Specifying type constraints for `Index`

Specify the default type for `Index` as `u16`. In addition, add ``WrappingsOps` and `From<u8>`.

```rust
type DefaultIdx = u16;
pub struct RingBufferTransient<Item, B, M, Index = DefaultIdx>
where
	Item: Codec + EncodeLike,
	B: StorageValue<(Index, Index), Query = (Index, Index)>,
	M: StorageMap<Index, Item, Query = Item>,
	Index: Codec + EncodeLike + Eq + WrappingOps + From<u8> + Copy,
{
	start: Index,
	end: Index,
	_phantom: PhantomData<(Item, B, M)>,
}
```

### 3. RingBuffer implementation

Now that we have the type definition for `RingBufferTransient` we need to write the implementation.

#### Initialize the transient

Specify how to create a new instance by creating a function called `new`, which makes use of the bounds `B` in storage to intialize the transient:

```rust
impl<Item, B, M, Index> RingBufferTransient<Item, B, M, Index>
where // ... same where clause as the type, elided here
{
	pub fn new() -> RingBufferTransient<Item, B, M, Index> {
		let (start, end) = B::get();
		RingBufferTransient {
			start,
			end,
			_phantom: PhantomData,
		}
	}
}
```

#### Implementing `RingBufferTrait`

To implement `RingBufferTrait`, write the following functions:

- `commit()`: to put the potentially changed bounds in storage
- `is_empty()`: to check whether the queue is empty to avoid expensive accesses to storage
- `push()`: to uphold the corresponding invariants from `is_empty()`.
- `pop()`: if the queue is not empty
  we `take` the value at `self.start` from storage, then increment `self.start` to point to the new first item of the queue
- `wrapping_add`: allows our ringbuffer to wrap around when reaching `max_value` of the `Index` type. The next step covers writing the `WrappingOps` trait declaration.

```rust
impl<Item, B, M, Index> RingBufferTrait<Item> for RingBufferTransient<Item, B, M, Index>
where
	Item: Codec + EncodeLike,
	B: StorageValue<(Index, Index), Query = (Index, Index)>,
	M: StorageMap<Index, Item, Query = Item>,
	Index: Codec + EncodeLike + Eq + WrappingOps + From<u8> + Copy,
{
	fn commit(&self) {
		B::put((self.start, self.end));
	}

	fn is_empty(&self) -> bool {
		self.start == self.end
	}

	fn push(&mut self, item: Item) {
		M::insert(self.end, item);
		// this will intentionally overflow and wrap around when bonds_end
		// reaches `Index::max_value` because we want a ringbuffer.
		let next_index = self.end.wrapping_add(1.into());
		if next_index == self.start {
			// queue presents as empty but is not
			// --> overwrite the oldest item in the FIFO ringbuffer
			self.start = self.start.wrapping_add(1.into());
		}
		self.end = next_index;
	}

	fn pop(&mut self) -> Option<Item> {
		if self.is_empty() {
			return None;
		}
		let item = M::take(self.start);
		self.start = self.start.wrapping_add(1.into());

		item.into()
	}
```

The `if` is necessary because we need to keep the invariant that `start == end` means that the queue
is empty, otherwise we would need to keep track of this state separately. Consequently, we "toss away" the
oldest item in the queue (if a new item is pushed into a full queue) by incrementing the start index.

#### The need for the `WrappingOps` trait

Since `std` does not provide a trait that allows the ringbuffer to be agnostic to the concrete `Index` type used. Therefore, we need to create our own trait for the types we want to support (`u8`, `u16`, `u32` and `u64`):

```rust
// There is no equivalent trait in std so we create one.
pub trait WrappingOps {
	fn wrapping_add(self, rhs: Self) -> Self;
	fn wrapping_sub(self, rhs: Self) -> Self;
}

macro_rules! impl_wrapping_ops {
	($type:ty) => {
		impl WrappingOps for $type {
			fn wrapping_add(self, rhs: Self) -> Self {
				self.wrapping_add(rhs)
			}
			fn wrapping_sub(self, rhs: Self) -> Self {
				self.wrapping_sub(rhs)
			}
		}
	};
}

impl_wrapping_ops!(u8);
impl_wrapping_ops!(u16);
impl_wrapping_ops!(u32);
impl_wrapping_ops!(u64);
```

#### Implementing the `Drop` trait

In order to make the usage more ergonomic and to avoid synchronization errors (where the storage map
diverges from the bounds) we also implement the
[`Drop` trait][drop-rustdocs]:

```rust
impl<Item, B, M, Index> Drop for RingBufferTransient<Item, B, M, Index>
where // ... same where clause elided
{
	fn drop(&mut self) {
		<Self as RingBufferTrait<Item>>::commit(self);
	}
}
```

On `drop`, we `commit` the bounds to storage. With this implementation of `Drop`, `commit` is called
when our transient goes out of scope, making sure that the storage state is consistent for the next
call to the using pallet.

## Examples

- [`ringbuffer-queue/src/lib.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/ringbuffer-queue/src/lib.rs): Shows a typical usage of the transient storage adapter.
- [`ringbuffer-queue/src/ringbuffer.rs`](https://github.com/substrate-developer-hub/recipes/tree/master/pallets/ringbuffer-queue/src/ringbuffer.rs): Contains an implementation of a transient storage adapter.

## References

#### How-to guides

- [Basic storage migration](nicks-migration)

#### Rust docs

- [trait objects](https://doc.rust-lang.org/book/ch17-02-trait-objects.html#trait-objects-perform-dynamic-dispatch)

[ringbuffer-wikipedia]: https://en.wikipedia.org/wiki/Circular_buffer
[fifo-wikipedia]: https://en.wikipedia.org/wiki/FIFO_(computing_and_electronics
[codec-rustdocs]: https://docs.rs/parity-scale-codec/1.3.0/parity_scale_codec/trait.Codec.html
[encodelike-rustdocs]: https://docs.rs/parity-scale-codec/1.3.0/parity_scale_codec/trait.EncodeLike.html
[phantomdata-rustdocs]: https://doc.rust-lang.org/std/marker/struct.PhantomData.html
[drop-rustdocs]: https://doc.rust-lang.org/std/ops/trait.Drop.html
