---
sidebar_position: 4
keywords: pallet design, intermediate, runtime
---

# Dispatchables, Events and Errors

_Write a dispatchable function that creates a Kitty capable of emitting its associated Event._

## Overview

In the previous section of this tutorial, we laid down the foundations geared to manage the ownership of our Kitties &mdash; even though they don't really exist yet! In this part of the tutorial, we'll be putting these foundations to use
by giving our pallet the ability to create a Kitty using the storage items we declared in the previous part. Breaking things down a little, we're going to:

- **Write `create_kitty`**: a dispatchable or publicly callable function allowing an account to mint a Kitty.
- **Write `mint()`**: a helper function that updates our pallet's storage items and performs error checks, called by `create_kitty`.
- **Include `Events`**: using FRAME's `#[pallet::events]` macro.

At the end of this part, we'll check that everything compiles without error and call our `create_kitty` extrinsic using the PolkadotJS Apps UI.

:::note
If you're feeling confident, you can continue building on your codebase from the previous part.
If you prefer using the "ACTION" items as a way to assist you through each step, replace the code from the
Part II with [this part's helper code][helper-code-pt3].
:::

## Learning outcomes

:arrow_right: Write a dispatchable function that updates storage items using a helper function.

:arrow_right: Write a private helper function with error handling

:arrow_right: Write and use pallet Events and Errors.

:arrow_right: Use PolkadotJS Apps UI to test pallet functionality.

## Steps

### 1. Public and private functions

Before we dive right in, it's important to understand the pallet design decisions we'll be making around coding up our Kitty pallet's minting and ownership management
capabilities.

As developers, we want to make sure the code we write is efficient and elegant. Oftentimes, optimizing for one optimizes for the other.
The way we're going to set up our pallet to optimize for both will be to break-up the "heavy lifting" dispatchable
functions into private helper functions. This improves code readability and reusability too. As we'll see, we can
create private functions which can be called by multiple dispatchable functions without compromizing on security. In fact, building this way can be considered an additive security feauture.

:::info
Check out [this how-to guide](/docs/basics/helper-functions/) about writing and using helper functions to learn more.
:::

Before jumping into implementing this approach, let's first paint the big picture of what combining dispatchables and helper functions looks like:

**`create_kitty`** (dispatchable function)

- check the origin is signed
- generate a random hash with the signing account
- create a new Kitty object using the random hash
- call a private `mint()` function
- increment the nonce using `increment_nonce()` from [Part II](/docs/tutorials/Kitties/Part%201/create-kitties#nonce)

**`mint`** (private helper function)

- check that the Kitty doesn't already exist
- update storage with the new Kitty ID (for all Kitties and for the owner's account)
- update the new total Kitty count for storage and the new owner's account
- deposit an Event to signal that a Kitty has succesfully been created

### 2. Write the `create_kitty` dispatchable

A [dispatchable][dispatchable-kb] in FRAME always follows the same structure. All pallet dispatchables live under the `#[pallet::call]` macro which requires declaring the dispatchables section with ` impl<T: Config> Pallet<T> {}`. Read the
[documentation][frame-macros-kb] on these FRAME macros to learn how they work. All we need to know here is that they're a useful feature of FRAME that minimizes the code required to write for pallets to be properly integrated in a Substrate chain's runtime.

#### Weights

As per the requirement for `#[pallet::call]` described in the its documentation, every dispatchable function must have an associated weight to it. Weights are
an important part of developing with Substrate as they provide safe-guards around the amount of computation to fit in a block at execution time.
[Substrate's weighting system][weights-kb] forces developers to think about the computational complexity each [extrinsic][extrinsics-kb] carries before it is called so that
a node will account for it's worst case, avoiding lagging the network with extrinsics that may take longer than the specified block time. Weights are also intimately linked to the [fee system][txn-fees-kb] for a signed extrinsic.

For this simple application, we're going to default all weights to 100.

Find ACTION #1 and replace it with the following code (we'll be compling it in the following section):

```rust
    let sender = ensure_signed(origin)?;

    let kitty_id = Self::mint(&sender, None, None)?;

    // Logging to the console
    log::info!("A kitty is born with ID: {:?}.", kitty_id);
}
```

:::note Why "DispatchResult" and not "DispatchResultWithPostInfo" ?
In `create_kitty` our return was of type `DispatchResultWithPostInfo`. Since `mint()` is a helper for `create_kitty`, we don't need to overwrite `PostDispatchInfo`,
so we can use a return type of [`DispatchResult`][dispatchresult-rustdocs] &mdash; its unaugmented version.
:::

### 3. Write the `mint()` function

As seen when we wrote `create_kitty` in the previous section, we'll need to create `mint()` for
writing our new unique Kitty object to the various storage items declared in Part II of this tutorial.

Let's get right to it. Our `mint()` function will take the following arguments:

- **`to`**: of type `&T::AccountId`
- **`dna`**: of type `Option<[u8; 16]>`
- **`gender`**: of type `Option<Gender>`

And it will return `Result<T::Hash, Error<T>>`.

Paste in the following code snippet to write the `mint` function, replacing ACTION #2 in the working codebase:

```rust
// Helper to mint a Kitty.
pub fn mint(
  owner: &T::AccountId,
  dna: Option<[u8; 16]>,
  gender: Option<Gender>,
) -> Result<T::Hash, Error<T>> {
  let kitty = Kitty::<T> {
    dna: dna.unwrap_or_else(Self::gen_dna),
    price: None,
    gender: gender.unwrap_or_else(Self::gen_gender),
    owner: owner.clone(),
  };

  let kitty_id = T::Hashing::hash_of(&kitty);

  // Performs this operation first as it may fail
  let new_cnt = Self::kitty_cnt().checked_add(1)
    .ok_or(<Error<T>>::KittyCntOverflow)?;

  // Performs this operation first because as it may fail
  <KittiesOwned<T>>::try_mutate(&owner, |kitty_vec| {
    kitty_vec.try_push(kitty_id)
  }).map_err(|_| <Error<T>>::ExceedMaxKittyOwned)?;

  <Kitties<T>>::insert(kitty_id, kitty);
  <KittyCnt<T>>::put(new_cnt);
  Ok(kitty_id)
}
```

Let's go over what the above code is doing.

The first thing we're doing is creating new values for a Kitty object. Then, we create a unique `kitty_id` using a hashing funciton on the adress of our Kitty object.

Next, we increment the `KittyCnt` using its gett function, checking for overflow.

Once we've done the check, we proceed with updating our storage items, making use of
the [`try_mutate`](https://substrate.dev/rustdocs/latest/frame_support/storage/trait.StorageMap.html#tymethod.try_mutate) and [`insert`][insert-rustdocs] methods from Substrate's StorageMap API and [`put`](https://substrate.dev/rustdocs/latest/frame_support/storage/trait.StorageValue.html#tymethod.put) from `StorageValue`.

:::note A quick recap of our storage items

- **`<Kitties<T>>`**: Stores a Kitty's unique traits and price, by storing the Kitty object.
- **`<KittyOwned<T>>`**: Keeps track of what accounts own what Kitty.
- **`<KittyCnt<T>>`**: A count of all Kitties in existence.
:::

### 4. Implement pallet Events

In Substrate, even though a transaction may be finalized, it does not necessarily imply that the function executed by that 
transaction fully succeeded. To verify this, we make our pallet emit an [Event][events-kb] at the end of the function. This not only 
reports the success of a function's execution, but also tells the "off-chain world" that some particular state transition has happened.

FRAME helps us easily manage and declare our pallet's events using the [`#[pallet::event]`][events-rustdocs] macro. With FRAME macros, events are just an enum declared like this:

```rust
#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config>{
    /// A function succeeded. [time, day]
    Success(T::Time, T::Day),
}
```

As you can see in the above snippet, we use:

`#[pallet::generate_deposit(pub(super) fn deposit_event)]` 

This allows us to deposit a specifc event using the pattern below:

```rust
Self::deposit_event(Event::Success(var_time, var_day));
```

In order to use events inside our pallet, we need to have the `Event` type declared inside our pallet's configuration trait, `Config`. Additionally &mdash; just as
when adding any type to our pallet's `Config` trait &mdash; we need to let our runtime know about it. 

This pattern is the same as when
we added the `KittyRandomness` type in [Part II of this tutorial](/docs/Tutorials/Kitties/Part%201/create-kitties#3-implement-on-chain-randomness) and has already been included from the initial scaffolding of our codebase: 

```rust
  /// Configure the pallet by specifying the parameters and types it depends on.
  #[pallet::config]
  pub trait Config: frame_system::Config {
      /// Because this pallet emits events, it depends on the runtime's definition of an event.
      type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
      //--snip--//
  }
```

:::note Notice that each event deposit is meant to be informative which is why it carries the various types associated with it.

It's good practice to get in the habit of documenting your event declarations so that your code is easy to read. It is convention to document events as such:

`/// Description. [types]`

Learn more about events [here][events-kb].
:::

Declare your pallet events by replacing the ACTION #3 line with:

```rust
  /// A new Kitty was sucessfully created. \[sender, kitty_id\]
  Created(T::AccountId, T::Hash),
  /// Kitty price was sucessfully set. \[sender, kitty_id, new_price\]
  PriceSet(T::AccountId, T::Hash, Option<BalanceOf<T>>),
  /// A Kitty was sucessfully transferred. \[from, to, kitty_id\]
  Transferred(T::AccountId, T::AccountId, T::Hash),
  /// A Kitty was sucessfully bought. \[buyer, seller, kitty_id, bid_price\]
  Bought(T::AccountId, T::AccountId, T::Hash, BalanceOf<T>),
```

We'll be using most of these events in Part IV of this tutorial. For now let's use the relevant event for our `create_kitty` dispatchable.

Complete it by replacing ACTION #4 with:

```rust
Self::deposit_event(Event::Created(to, kitty_id));
```
:::note
If you're building your codebase from the previous part (and haven't been using the helper file for this part) you'll need to add `Ok(())` and properly close the `create_kitty` dispatchable.
:::

### 5. Error handling 

FRAME provides us with an error handling system using [`[#pallet::errors]`][errors-kb] which allows us to specify errors for our pallet and use them across our pallet's functions. 

Declare all possible errors using the provided FRAME macro under `#[pallet::error]` (replace line ACTION #5a):

```rust
		/// Handles arithemtic overflow when incrementing the Kitty counter.
		KittyCntOverflow,
		/// An account cannot own more Kitties than `MaxKittyCount`.
		ExceedMaxKittyOwned,
		/// Buyer cannot be the owner.
		BuyerIsKittyOwner,
		/// Cannot transfer a kitty to its owner.
		TransferToSelf,
		/// Handles checking whether the Kitty exists.
		KittyNotExist,
		/// Handles checking that the Kitty is owned by the account transferring, buying or setting a price for it.
		NotKittyOwner,
		/// Ensures the Kitty is for sale.
		KittyNotForSale,
		/// Ensures that the buying price is greater than the asking price.
		KittyBidPriceTooLow,
		/// Ensures that an account has enough funds to purchase a Kitty. 
		NotEnoughBalance,
```

We'll be using these errors once we write the interactive functions in the next section. Notice that we've already used `KittyCntOverflow` and `ExceedMaxKittyOwned` in our `mint` function.

Now's a good time to see if your chain can compile. Instead of only checking if your pallet compiles, run the following command to see if everything can build:

```rust
cargo +nightly build --release
```
:::tip
If you ran into errors, scroll to the first error message in your terminal, identify what line
is giving an error and check whether you've 
followed each step correctly. Sometimes a mismatch of curly brackets will unleash a whole bunch 
of errors that are difficult to understand &mdash; double check your code!
:::

Did that build fine? Congratulations! That's the core functionality of our Kitties pallet. In the next step you'll be able to 
see everything you've built so far in action.

### 6. Testing with PolkadotJS Apps

Assuming that you successfully built your chain, let's run it and use the [PolkadotJS Apps UI](https://polkadot.js.org/apps/#/explorer) to interact with it.

In your chain's project directory, run:

```bash
./target/release/node-kitties --tmp --dev
```

By doing this, we're specifying to run a temporary chain in developer mode, so as not to need to purge storage each time we want to start a fresh chain.

Assuming that blocks are being finalized (which you should be able to see from your terminal in which you ran the above command), head over to [Poladot.js Apps][polkadotjsapps].

**Follow these steps:**

1. Check that you're connected to Local Node, under "Development". Your node will default to `127.0.0.1.:9944`.
2. Tell the UI about your custom types. 
This requires you to paste them into the "_Settings_" -> "_Developers_" section.
3. Go to "_Developer_" -> "_Extrinsics_". Paste this in the JSON code editor:

```json
{
  "Gender": {
    "_enum": [ "Male", "Female"]
  },
  "Kitty": {
    "dna": "[u8; 16]",
    "price": "Option<Balance>",
    "gender": "Gender",
    "ownder": "AccountId"
  }
}
```

> The reason we need this is because we created types that PolkadotJS Apps isn't designed to read custom types by default. By adding them, it can
properly decode each of our storage items that rely on custom types. Add this in a file caleld `types.json` in your projects `runtime` folder.

3. Now go to: _"Developer"_ -> _"Extrinsics"_ and submit a signed extrinsic using _substrateKitties_ by calling the `createKitty()` dispatchable. Make 3 different transactions from Alice, Bob and Charlie's accounts
4. Check for the associated event _"Created"_ by going to "_Network_" -> "_Explorer_". You should be able to see the event emitted and query its block details.
5. Check your newly created Kitty's details by going to "_Developer_" -> "_Chain State_". Select the _substrateKitties_ pallet and query `Kitties(Hash): Kitty`. **Note:** You'll notice that this is actually querying all of your pallet's storage items!

Be sure to uncheck the "include option" box and you should be able to see the details of your newly minted Kitty in the following format:

```json
kitties.kitties: Option<Kitty>
[
  [
    [
      0x15cb95604033af239640125a30c45b671a282f3ef42c6fc48a78eb18464b30a9
    ],
    {
      dna: 0xaf2f2b3f77e110a56933903a38cde1eb,
      price: null,
      gender: Female,
      ownder: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
    }
  ]
]
```

5. Check that other storage items correctly reflect the creation of additional Kitties.

:::note Congratulations!
You're pretty much able to take it from here at this point! We've learnt how to implement the key parts of what powers a FRAME pallet and how to put them to use. All part IV of this tutorial covers is adding more capabilities to our pallet by taking what we've learnt in this part.

To recap, in this part of the tutorial you've learnt how to:

- Distinguish between implementing a dispatchable function and a private helper function.
- Use `#[pallet::call]`, `#[pallet::events]` and `#[pallet::error]`.
- Implement basic error checking with FRAME.
- Update values in storage with safety checks.
- Implement FRAME events and use them in a function.
- Query storage items and chain state using the PolkadotJS Apps UI.
:::

## Next steps

- Create a dispatchable to buy a Kitty
- Create a dispatchable to transfer a Kitty
- Create a dispatchable to breed two Kitties

[frame-macros-kb]: https://substrate.dev/docs/en/knowledgebase/runtime/macros#palletcall
[txn-fees-kb]: https://substrate.dev/docs/en/knowledgebase/runtime/fees
[weights-kb]: https://substrate.dev/docs/en/knowledgebase/learn-substrate/weight
[contains-key-rustdocs]: https://substrate.dev/rustdocs/latest/frame_support/storage/trait.StorageMap.html#tymethod.contains_key
[insert-rustdocs]: https://substrate.dev/rustdocs/latest/frame_support/storage/trait.StorageMap.html#tymethod.insert
[storage-value-rustdocs]: https://substrate.dev/rustdocs/latest/frame_support/storage/types/struct.StorageValue.html#method.put
[storagemap-rustdocs]: https://substrate.dev/rustdocs/latest/frame_support/storage/types/struct.StorageMap.html#method.insert
[events-rustdocs]: https://crates.parity.io/frame_support/attr.pallet.html#event-palletevent-optional
[events-kb]: https://substrate.dev/docs/en/knowledgebase/runtime/events
[polkadotjsapps]: https://polkadot.js.org/apps/#/explorer
[dispatchresult-rustdocs]: https://substrate.dev/rustdocs/latest/frame_support/dispatch/type.DispatchResult.html
[dispatchable-kb]: https://substrate.dev/docs/en/knowledgebase/getting-started/glossary#dispatch
[extrinsics-kb]:  https://substrate.dev/docs/en/knowledgebase/runtime/execution#executing-extrinsics
[helper-code-pt3]: https://github.com/substrate-developer-hub/substrate-how-to-guides/blob/main/static/code/kitties-tutorial/03-dispatchables-and-events.rs
[errors-kb]: https://substrate.dev/docs/en/knowledgebase/runtime/errors