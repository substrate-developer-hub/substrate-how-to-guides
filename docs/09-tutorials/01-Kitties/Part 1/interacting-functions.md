---
sidebar_position: 5
keywords: pallet design, intermediate, runtime
theme: 
code: ../static/code/kitties-tutorial/04-interacting-functions.rs
---

# Interacting with your Kitties

_Add pallet capabilities that unleash the potential of your Substrate Kitty application._

## Overview

Up until this point in the tutorial, we've built a chain capable of only creating and tracking the ownership of Kitties. In this part of the tutorial, we want to make our runtime more
like a game by introducing other functions like buying and
selling Kitties. In order to achieve this, we'll first need to enable users
to update the price of their Kitty. Then we can add functionality to enable users to transfer, buy and breed Kitties.
## Learning outcomes

:arrow_right: Learn how to create a dispatchable that updates an object in storage.

:arrow_right: Getting a value from a struct in storage.

:arrow_right: How to use the `transfer` from FRAME's Currency trait.

:arrow_right: How to write sanity check using `ensure!()`.

## Steps

### 1. Set a price for each Kitty

In [the helper file for this part of the tutorial](https://github.com/substrate-developer-hub/substrate-how-to-guides/blob/main/static/code/kitties-tutorial/04-interacting-functions.rs), you'll notice that the structure of `set_price` is already laid out.

Your job is to replace ACTION lines #1, #2 and #3 lines with what you'll learn in sections A-D below. 
#### A. Checking Kitty owner

As we create functions which modify objects in storage, we
should always check that only the appropriate users are successful when calling those dispatchable functions.

The general pattern for an ownership check will look something like this:

```rust
let owner = Self::owner_of(object_id).ok_or("No owner for this object")?;

ensure!(owner == sender, "You are not the owner");
```

**Your turn!** Paste in this code snippet to replace ACTION #1:

```rust
ensure!(Self::is_kitty_owner(&kitty_id, &sender)?, <Error<T>>::NotKittyOwner);
```
#### B. Updating the price of our Kitty object

Every Kitty object has a price attribute that we've set to [`None`] as a default value inside the
`mint` function in [Part II](/docs/tutorials/Kitties/Part%201/dispatchables-and-events#3-write-the-mint-function):

```rust
let kitty = Kitty::<T> {
				dna: dna.unwrap_or_else(Self::gen_dna),
				price: None,                           //<-- 👀 here
				gender: gender.unwrap_or_else(Self::gen_gender),
				owner: owner.clone(),
			};
```

To update the price of a Kitty, we'll need to:

- Get the Kitty object in storage.
- Update the object with the new price.
- Push it back into storage.

Changing a value in an existing object in storage would be written in the following way:

```rust
let mut object = Self::get_object(object_id);
object.value = new_value;

<Object<T>>::insert(object_id, object);
```

:::note
Rust expects you to declare a variable as mutable (using the `mut` keyword) whenever its value is going to be updated.
:::

**Your turn!** Paste in the following snippet to replace the ACTION #2 line: 

```rust
kitty.price = new_price.clone();
<Kitties<T>>::insert(&kitty_id, kitty);
```

#### D. Deposit an event

Once all checks are passed and the new price is written to storage, we can deposit an event 
[just like we did in Part III](/docs/tutorials/Kitties/Part%201/dispatchables-and-events#4-implement-pallet-events). 
Replace the line marked as ACTION #3 with:

 ```rust
 Self::deposit_event(Event::PriceSet(sender, kitty_id, new_price));
 ```

 Now whenever the `set_price` dispatchable is called successfully, it will emit a `PriceSet` event. 🎉

### 2. Transfer a Kitty

You already have the tools and knowledge you'll need to create the transfer functionality from [step 1](#1-set-a-price-for-each-kitty). The main difference is that there are **two parts** to achieving this:

1. A **dispatchable function** called `transfer()`: this is a publicly callable dispatchable exposed by your pallet.
2. A **private function** called `transfer_kitty_to()`: this will be a private helper function called by `transfer()` to handle all storage updates when transferring a Kitty.

Separating the logic this way makes the private `transfer_kitty_to()` function reusable
by other dispatchable functions of our pallet, without needing to duplicate code. In our case, we're going to reuse it for
the `buy_kitty` dispatchable we're creating in the next section.

#### `transfer` 

Paste in the following snippet to replace ACTION #5 in the template code:

```rust
#[pallet::weight(100)]
pub fn transfer(
    origin: OriginFor<T>, 
    to: T::AccountId, 
    kitty_id: T::Hash
) -> DispatchResult {
    let from = ensure_signed(origin)?;

    // Ensure the kitty exists and is called by the kitty owner
    ensure!(Self::is_kitty_owner(&kitty_id, &from)?, <Error<T>>::NotKittyOwner);

    // Verify the kitty is not transferring back to its owner.
    ensure!(from != to, <Error<T>>::TransferToSelf);

    // Verify the recipient has the capacity to receive one more kitty
    let to_owned = <KittiesOwned<T>>::get(&to);
    ensure!((to_owned.len() as u32) < T::MaxKittyOwned::get(), <Error<T>>::ExceedMaxKittyOwned);

    Self::transfer_kitty_to(&kitty_id, &to)?;

    Self::deposit_event(Event::Transferred(from, to, kitty_id));

    Ok(())
}
```

By now the above pattern should be familiar. We always check that the transaction is signed; then we verify that the Kitty 
being transfer is owned by the sender of this transaction; and last we call the `transfer_kitty_to` helper to update 
all storage items appropriately.

#### `transfer_kitty_to` 

Now, the `transfer_kitty_to` function will be a helper to perform all storage updates once a Kitty has been bought and sold.
All it needs to do is perform safety checks and update the following storage items:

- `KittiesOwned`: to update the owner of the Kitty.
- `Kitties`: to reset the price in the Kitty object to None.

Copy the following to replace ACTION #6:

```rust
#[transactional]
pub fn transfer_kitty_to(
    kitty_id: &T::Hash,
    to: &T::AccountId,
) -> Result<(), Error<T>> {
    let mut kitty = Self::kitties(&kitty_id).ok_or(<Error<T>>::KittyNotExist)?;

    let prev_owner = kitty.owner.clone();

    // Remove `kitty_id` from the KittyOwned vector of `prev_kitty_owner`
    <KittiesOwned<T>>::try_mutate(&prev_owner, |owned| {
        if let Some(ind) = owned.iter().position(|&id| id == *kitty_id) {
            owned.swap_remove(ind);
            return Ok(());
        }
        Err(())
    }).map_err(|_| <Error<T>>::KittyNotExist)?;

    // Update the kitty owner
    kitty.owner = to.clone();
    // Reset the ask price so the kitty is not for sale until `set_price()` is called
    // by the current owner.
    kitty.price = None;

    <Kitties<T>>::insert(kitty_id, kitty);

    <KittiesOwned<T>>::try_mutate(to, |vec| {
        vec.try_push(*kitty_id)
    }).map_err(|_| <Error<T>>::ExceedMaxKittyOwned)?;

    Ok(())
}
```

Notice the use of [`[#transactional]`](https://substrate.dev/rustdocs/latest/frame_support/attr.transactional.html) which we imported at the very beginning of this tutorial. It allows us to write dispatchable functions that will only write to storage at the same time as the helper functions it calls, making sure all storage writes happen together.

### 3. Buy a Kitty

#### A. Check a Kitty is for Sale

We'll need to ensure 2 things before we can allow the user of this function to purchase a Kitty: first, check that the 
Kitty is for sale; and second, check whether the Kitty's current price is within the user's budget and whether the user has 
enough free balance.

Replace line ACTION #7:

```rust
// Check the kitty is for sale and the kitty ask price <= bid_price
if let Some(ask_price) = kitty.price {
    ensure!(ask_price <= bid_price, <Error<T>>::KittyBidPriceTooLow);
} else {
    Err(<Error<T>>::KittyNotForSale)?;
}

// Check the buyer has enough free balance
ensure!(T::Currency::free_balance(&buyer) >= bid_price, <Error<T>>::NotEnoughBalance);
```

In a similar vain, we have to verify whether the user has the capacity to receive a Kitty &mdash; remember we're using 
a [`BoundedVec`](https://substrate.dev/rustdocs/latest/frame_support/storage/bounded_vec/struct.BoundedVec.html) that can 
only hold a fixed number of Kitties, defined in our pallet's `MaxKittyOwned` constant.

One last check before we can allow this user to call this dispatchable (paste this in following the last snippet):

```rust
// Verify the buyer has the capacity to receive one more kitty
let to_owned = <KittiesOwned<T>>::get(&buyer);
ensure!((to_owned.len() as u32) < T::MaxKittyOwned::get(), <Error<T>>::ExceedMaxKittyOwned);

let seller = kitty.owner.clone();
```

#### B. Making a Payment

In [Step 2](#2-transfer-a-kitty), we added the functions necessary to transfer the _ownership_ of our
Kitties. But we haven't yet touched on the currrency associated to our pallet.
In this step we'll learn how to use [FRAME's Currency trait][currency-frame-rustdocs] to adjust account balances
using its very own [`transfer` method][transfer-currency-rustdocs]. It's useful to understand why it's important to use the `transfer` method in particular and how we'll be accessing it:

- The reason we'll be using it is to ensure our runtime has the same understanding of currency throughout the pallets
  it interacts with. The way that we ensure this is to use the `Currency` trait
  from `frame_support`.

- Conveniently, it handles a
  [`Balance`][currency-balances-rustdocs] type, making it compatible with `BalanceOf` type we created for `kitty.price`. Take a look at how the `transfer`
  function we'll be using is structured (from the [Rust docs][currency-transfer-rustdocs]):

```rust
fn transfer(
    source: &AccountId,
    dest: &AccountId,
    value: Self::Balance,
    existence_requirement: ExistenceRequirement
) -> DispatchResult
```
Now we can make use of the `Currency` type in our pallet's `Config` trait and `ExistenceRequirement` &ndash; that we 
[initially started with in Part I](/docs/tutorials/Kitties/Part%201/basic-setup#2-write-out-pallet_kitties-scaffold).

Update the balances of both the caller of this function and the receiver, replacing ACTION #8:

```rust
// Transfer the amount from buyer to seller
T::Currency::transfer(&buyer, &seller, bid_price, ExistenceRequirement::KeepAlive)?;

// Transfer the kitty from seller to buyer
Self::transfer_kitty_to(&kitty_id, &buyer)?;

// Deposit relevant Event
Self::deposit_event(Event::Bought(buyer, seller, kitty_id, bid_price));
```

### 4. Breed Kitties

The logic behind breeding two Kitties is to multiply each corresponding DNA segment from two Kitties,
which will produce a new DNA sequence. Then, that DNA is used when minting a new Kitty. This helper function is already 
provided for you in the template file for this section.

Paste in the following to complete the `breed_kitty` function, replacing line ACTION #10:

```rust
let new_dna = Self::breed_dna(&kid1, &kid2)?;
```

Now that we've used the user inputs of Kitty IDs and combined them to create a new unique Kitty ID, we can
use the `mint()` function to write that new Kitty to storage. Replace line ACTION #11:

```rust
Self::mint(&sender, Some(new_dna), None)?;
```

### 5. Genesis configuration

The final step before our pallet is ready to be used is to set the genesis state of our storage items. We'll make use of
FRAME's `[pallet::genesis_config]` to do this. Essentially, we're declaring what the Kitties object in storage contains 
in the genesis block. Copy the following code to replace ACTION #12: 

```rust
// Our pallet's genesis configuration.
#[pallet::genesis_config]
pub struct GenesisConfig<T: Config> {
    pub kitties: Vec<(T::AccountId, [u8; 16], Gender)>,
}

// Required to implement default for GenesisConfig.
#[cfg(feature = "std")]
impl<T: Config> Default for GenesisConfig<T> {
    fn default() -> GenesisConfig<T> {
        GenesisConfig { kitties: vec![] }
    }
}

#[pallet::genesis_build]
impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
    fn build(&self) {
        // When building a kitty from genesis config, we require the dna and gender to be supplied.
        for (acct, dna, gender) in &self.kitties {
            let _ = <Pallet<T>>::mint(acct, Some(dna.clone()), Some(gender.clone()));
        }
    }
}
```

To let our chain know about our pallet's genesis configuration, we need to modify the `chain_spec.rs` file in our project's `node` folder. Go to `/node/src/chain_spec.rs` and add the following inside the `testnet_genesis` function:

```rust
//-- snip
		kitties: KittiesConfig {
			kitties: vec![],
		},
//-- snip
```
### 6. Update `runtime/src/lib.rs` and interact with your Kitties

If you've completed all of the preceding parts and steps of this tutorial, you're
all geared up to run your chain and start interacting with all the new capabilities of your Kitties pallet.

Build and run your chain using the following commands:

```bash
cargo build --release
./target/release/node-kitties --dev --tmp
```

Now check your work using the Polkadot-JS Apps UI just like [we did in the previous part](/docs/Tutorials/Kitties/Part%201/dispatchables-and-events#5-testing-with-polkadotjs-apps). Once your chain is running and connected to the PolkadotJS Apps UI, perform these manual checks:

- Fund multiple users with tokens so they can all participate
- Have each user create multiple Kitties
- Try to transfer a Kitty from one user to another using the right and wrong owner
- Try to set the price of a Kitty using the right and wrong owner
- Buy a Kitty using an owner and another user
- Use too little funds to purchase a Kitty
- Overspend on the cost of the Kitty and ensure that the balance is reduced appropriately
- Breed a Kitty and check that the new DNA is a mix of the old and new

After all of these actions, confirm that all users have the right number of Kitties, the total Kitty count is correct, and any other storage variables are correctly represented

:::note Congratulations!

You've successfully created the backend of a fully functional Substrate chain capable of creating and managing Substrate Kitties. It could also be abstracted to other NFT-like use cases. Most importantly, at this point in the tutorial you should have all the knowledge you need to start creating your own pallet logic and dispatchable functions.
:::

## Next steps

Complete Part II of this tutorial to:

- Connect your chain to the front-end template
- Customize the template using PolkadotJS API
- Interact with kitty avatars using a custom front-end React app

[transfer-currency-rustdocs]: https://crates.parity.io/frame_support/traits/tokens/currency/trait.Currency.html#tymethod.transfer
[frame-balances-rustdocs]: https://crates.parity.io/frame_support/traits/tokens/currency/trait.Currency.html
[polkadotjs-ui]: https://polkadot.js.org/apps/#/explorer
[rust-u8]: https://doc.rust-lang.org/std/primitive.u8.html
[currency-frame-rustdocs]: https://substrate.dev/rustdocs/latest/frame_support/traits/tokens/currency/index.html
[currency-balances-rustdocs]: https://substrate.dev/rustdocs/latest/frame_support/traits/tokens/currency/trait.Currency.html#associatedtype.Balance
[balances-frame]: https://substrate.dev/rustdocs/latest/pallet_balances/index.html
[currency-transfer-rustdocs]: https://substrate.dev/rustdocs/latest/frame_support/traits/tokens/currency/trait.Currency.html#tymethod.transfer