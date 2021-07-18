---
sidebar_position: 4
keywords: pallet design, intermediate, runtime
---

# Part III: Dispatchables and Events
_Write a dispatchable function that creates a Kitty capable of emitting its associated Event._

## Learning outcomes

:arrow_right: Write a dispatchable function that updates storage items using a helper function.

:arrow_right: Write a private helper function with error handling

:arrow_right: Write and use pallet Events.

:arrow_right: Use PolkadotJS Apps UI to test pallet functionality.

## Overview
In the previous section of this tutorial, we laid down the foundations geared to manage the ownership of our Kitties &mdash; even though they don't really exist yet! In this part of the tutorial, we'll be putting these foundations to use 
by giving our pallet the ability to create a Kitty using the storage items we declared in the previous part 
of this tutorial. Breaking things down a little, we're going to:
- **write `create_kitty`**: a dispatchable or publicly callable function allowing an account to mint a Kitty.
- **write `mint()`**: a helper function that updates our pallet's storage items and performs error checks, called by `create_kitty`.
- **include `Events`**: using FRAME's `#[pallet::events]` macro.

At the end of this part, we'll check that everything compiles without error and call our `create_kitty` extrinsic using the PolkadotJS Apps UI.

## Steps

### 1. Public and private functions

Before we dive right in, it's important to understand the pallet design decisions we'll be making around coding up our Kitty pallet's minting and ownership management
capabilities.

As developers, we want to make sure the code we write is efficient and elegant. Oftentimes, optimizing for one optimizes for the other.
The way we're going to set up our pallet up to achieve elegance and efficiency will be to break-up the "heavy lifting" dispatchable 
functions or extrinsics into private helper functions. This improves code readability and reusability too. As we'll see, we can
create private functions which can be called by multiple dispatchable functions without compromizing on security.

:::info 
Check out [this how-to guide](docs/basics/basic-pallet-integration/) about writing and using helper functions to learn more.
:::

In the next section, we'll go over what implementing this approach looks like. For our immediate purposes, let's paint the big picture of what this means for the logical flow of our pallet:

**`create_kitty`** 
- check the origin is signed
- generate a random hash with the signing account 
- create a new Kitty object using the random hash
- call a private `mint()` function
- increment the nonce using `increment_nonce()` from the previous part

**`mint`**
- check that the Kitty doesn't already exist
- update storage with the new Kitty ID (for all Kitties and for the owner's account)
- update the new total Kitty count for storage and the new owner's account
- deposit an Event to signal that a Kitty has succesfully been created

### 2. Write the `create_kitty` dispatchable 

A dispatchable in FRAME always follows the same structure. All pallet dispatchables live under the `#[pallet::call]` macro which requires declaring the dispatchables section with `    impl<T: Config> Pallet<T> {}`. Read the 
[documentation][frame-macros-kb] on these FRAME macros to learn how they work. All we need to know here is that they're a useful feature of FRAME that minimizes the code required to write for pallets to be properly integrated in a Substrate chain's runtime.

#### Weights
As per the requirement for `#[pallet::call]` described in the its documentation, every dispatchable function must have an associated weight to it. Weights are
an important part of developing with Substrate as they provide safe-guards around the amount of computation to fit in a block at execution time. 
[Substrate's weighting system][weights-kb] forces developers to think about the computational complexity each extrinsic carries before it is called so that 
a node will account for it's worst case, avoiding a lagging the network with extrinsics that take longer than a single block time. Also, weights are intimately linked to the fee system for a signed extrinsic. Learn more about this [here][txn-fees-kb].

:::tip Your turn!
Use the template code for this section to
help you write the `create_kitty` dispatchable. 
No need to worry about the `mint` function, we'll be going over that in the next section.
:::

Assuming you've correctly declared the macros required for the dispatchable, you should be able
to prove to yourself that everything is working correctly by running:

```bash
cargo build -p pallet-kitties
```

### 3. Write the `mint()` function

As seen when we wrote `create_kitty` in the previous section, we'll need  `mint` for 
writing our new unique Kitty object to the various  storage items declared in part II of this tutorial.

:::note A quick recap of our storage items

- **`<Kitties<T>>`**: Stores a Kitty's unique traits and price, by storing the Kitty object.
- **`<KittyOwner<T>>`**: Keeps track of what accounts own what Kitty.
- **`<AllKittiesArray<T>>`**: An index to track of all Kitties.
- **`<AllKittiesCount<T>>`**: Stores the total amount of Kitties in existence.
- **`<AllKittiesIndex<T>>`**: Keeps track of all the Kitties.
- **`<OwnedKittiesArray<T>>`**: Keep track of who a Kitty is owned by.
- **`<OwnedKittiesCount<T>>`**: Keeps track of the total amount of Kitties owned.
- **`<OwnedKittiesIndex<T>>`**: Keeps track of all owned Kitties by index.
:::

---

Let's get right to it. Our `mint()` function will take the following arguments:
- **`to`**: of type `T::AccountId`
- **`kitty_id`**: of type `T::Hash`
- **`new_kitty`**: of type `Kitty<T::Hash, T::Balance>`

And it will return `DispatchResult`.

:::note Why "DispatchResult" and not "DispatchResultWithPostInfo" ?
In `create_kitty` our return was of type `DispatchResultWithPostInfo`. Since `mint()` is a helper for `create_kitty`, we don't need to overwrite `PostDispatchInfo`,
so we can use a return type of [`DispatchResult`][dispatchresult-rustdocs] &mdash; its unaugmented version.
:::

:::tip Your turn! 
Write out the skeleton of the `mint()` function. 

**HINT:** it will go towards the bottom of your pallet, under `impl<T: Config> Pallet<T> {}` 
:::

Now that we have the skeleton of the `mint()` function, let's go over what to write inside it. 

The first thing we'll need to do is to check that the Kitty being passed in doesn't already exist. To accomplish this, we can use the built-in `ensure!` macro that Rust provides us, along with
a method provided by FRAME's `StorageMap` called `contains_key`. 

>[`contains_key`][contains-key-rustdocs] will check if a key matches the Hash value in an existing Kitty object. And 
`ensure!` will return an error if the storage map already
contains the given Kitty ID. 

The check can be written like this:
```rust
ensure!( !<SomeStorageMapStruct<T>>::contains_key(some_key), 
"SomeStorageMapStruct already contains_key");
```

Once we've done the check, we can proceed with updating our storage items with the Kitty object passed into our function call. To do this, we'll make use of
the [`insert`][insert-rustdocs] method from our StorageMap API, using the following pattern:

```rust
<SomeStorageMapStruct<T>>::insert(some_key, new_key);
```

Finally, we'll need to compute a few variables to update our storage items which keep track of:
- the indices and count for all Kitties. 
- the indices and count of owned Kitties.

All this requires us to do is add 1 to the current values held by `<AllKittiesCount<T>>` and `<OwnedKittiesCount<T>>`. We can use the same pattern as we did in the previous part when we created `increment_nonce`, using Rust's `checked_add` and `ok_or`:

```rust
let new_value = previous_value.checked_add(1).ok_or("Overflow error!");
```

:::tip Your turn!
Write the remaining "storage-write" operations 
for the `mint()` function.

**HINT:** There's 8 in total. And `StorageValue` has a different method than `StorageMap` to update its storage instance &mdash; have a glance at the [methods it exposes][storage-value-rustdocs] to learn more.
:::

### 4. Implement pallet Events

In Substrate, even though a transaction may be finalized, it does not necessarily imply that the function executed by that transaction fully succeeded.

To verify this, we make our pallet emit an Event at the end of the function. This not only report the success of a function's execution, but also tells the "off-chain world" that some particular state transition has happened.

FRAME helps us easily manage and declare our pallet's events using the [`#[pallet::event]`][events-rustdocs] macro. With FRAME macros, events are just an enum declared like this:

```rust
#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config>{
    /// A function succeeded. [time, day] 
    Success(T::Time, T::Day),
}
```
As you can see in the above snippet, we use `#[pallet::generate_deposit(pub(super) fn deposit_event)]` which allows us to deposit a 
specifc event using pattern below:

```rust
Self::deposit_event(Event::Success(var_time, var_day)); 
```
In order to use events inside our pallet, we need to add the `Event` type to our pallet's configuration trait, `Config`. Additionally &mdash; just as 
when adding any type to our pallet's `Config` trait &mdash; we need to let our runtime know about it. This pattern is the same as when
we added the `KittyRandomness` type in [part II of this tutorial](/docs/Tutorials/Kitties/create-kitties#2-implementing-randomness).

:::tip Your turn!
Set your pallet up for handling events by adding the `Event` type to both your pallet's configuration trait and runtime implementation.
Then, write an event for `mint()` with the appropriate return types.

**HINT #1:** Once all storage updates have been made in `mint()`, we want to inform the external world which account 
has created the Kitty, as well as what that Kitty's ID is.

**HINT #2:** The ubiquitous Event type is: `type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;`.

:::

:::note Notice that each event deposit is meant to be informative which is why it carries the various types associated with it. 

It's good practice to get in the habit of documenting your event declarations so that your code is easy to read. It is convention to document events as such: 

`/// Description. [types]`

Learn more about events [here][events-kb].
:::

Now's a good time to see if your chain can compile. Instead of only checking if your pallet compiles, run the following command to see if everything can build:

```rust
cargo build --release
```
### 5. Testing with PolkadotJS Apps

Assuming that you successfully built your chain, let's run it and use the PolkadotJS Apps UI to interact with it.

In your chain's project directory, run:

```bash
./target/release/kitties-node --tmp --dev
```

By doing this, we're specifying to run a temporary chain in developer mode, so as not to need to purge storage each time we want to start a fresh chain. 

Assuming that blocks are being finalized (which you should be able to see from your terminal in which you ran the above command), head over to [Poladot.js Apps][polkadotjsapps]. 

**Follow these steps:**

1. Check that you're connected to Local Node, under "Development".
2. Tell the UI about your custom types. This requires you to paste them into the "*Settings*" -> "*Developers*" section. 
2. Go to "*Developer*" -> "*Extrinsics*". Paste this in the JSON code editor:

```json
{
  "AccountInfo": {
    "nonce": "Index",
    "consumers": "RefCount",
    "providers": "RefCount",
    "data": "AccountData"
  },
  "Address": "MultiAddress",
  "LookupSource": "AccountId",
  "Gender": {
    "_enum": [
      "male",
      "female"
    ]
  },
  "Kitty": {
    "id": "H256",
    "dna": "H256",
    "price": "Balance",
    "gender": "Gender"
  }
}
```

The reason we need this is because we created types that PolkadotJS Apps isn't designed to read. By adding them, it's capable to properly decode each of our storage items that use custom types.

3. Submit a signed extrinsic using *substrateKitties* by calling the `createKitty()` dispatchable.
4. Check for the associated event by going to "*Network*" -> "*Explorer*". You should be able to see the event emitted and query its block details.
5. Check your newly created Kitty's details by going to "*Developer*" -> "*Chain State*". Select the *substrateKitties* pallet and query `Kitties(Hash): Kitty`.

You should be able to see your newly minted Kitty (or at least what it's made of!):

```json
substrateKitties.kitties: Kitty
[
    [
      0xf78c3e9c10498c350d639a10fa773cb878b85bf2c5989e38f8b1cd5c0bafb7ee
    ],
    {
      id: 0xf78c3e9c10498c350d639a10fa773cb878b85bf2c5989e38f8b1cd5c0bafb7ee,
      dna: 0xf78c3e9c10498c350d639a10fa773cb878b85bf2c5989e38f8b1cd5c0bafb7ee,
      price: 0,
      gender: female
    }
  ],
```
5. Check that other storage items correctly reflect the creation of  additional Kitties.


:::note Congratulations!
You're pretty much able to take it from here at this point! We've learnt how to implement the key parts of what powers a FRAME pallet and how to put them to use. All part IV of this tutorial covers is adding more capabilities to our pallet by taking what we've learnt in this part.  

To recap, in this part of the tutorial you've learnt how:
- To distinguish between implementing a dispatchable function and a private helper function.
- To use `#[pallet::call]` and `#[pallet::events]`.
- To do basic error checking.
- To update values in storage.
- To implement events and use them in a function.
- To query storage items and chain state using the PolkadotJS Apps UI.
:::
## Next steps
- Create a dispatchable to buy a Kitty
- Create a dispatchable to transfer a Kitty
- Create a dispatchable to breed two Kitties

[frame-macros-kb]: https://substrate.dev/docs/en/knowledgebase/runtime/macros#palletcall
[txn-fees-kb]: https://substrate.dev/docs/en/knowledgebase/runtime/fees
[weights-kb]: https://substrate.dev/docs/en/knowledgebase/learn-substrate/weight
[contains-key-rustdocs]: https://substrate.dev/rustdocs/v3.0.0/frame_support/storage/trait.StorageMap.html#tymethod.contains_key
[insert-rustdocs]: https://substrate.dev/rustdocs/v3.0.0/frame_support/storage/trait.StorageMap.html#tymethod.insert
[storage-value-rustdocs]: https://substrate.dev/rustdocs/v3.0.0/frame_support/storage/types/struct.StorageValue.html#method.put
[events-rustdocs]: https://crates.parity.io/frame_support/attr.pallet.html#event-palletevent-optional
[events-kb]: https://substrate.dev/docs/en/knowledgebase/runtime/events
[polkadotjsapps]: https://polkadot.js.org/apps/#/explorer
[dispatchresult-rustdocs]: https://substrate.dev/rustdocs/v3.0.0/frame_support/dispatch/type.DispatchResult.html