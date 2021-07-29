---
sidebar_position: 5
keywords: pallet design, intermediate, runtime
---

# Part IV: Interacting with your Kitties

_Add pallet capabilities that unleash the potential of your Substrate Kitty dApp._

## Learning outcomes

:arrow_right: Learn how to create a dispatchable that updates an object in storage.

:arrow_right: Getting a value from a struct in storage.

:arrow_right: How to use the `transfer` from FRAME's Currency trait.

:arrow_right: How to write sanity check using `ensure!()`.

## Overview

Up until this point in the tutorial, we've built a chain capable of only creating and tracking the ownership of Kitties. In this part of the tutorial, we want to make our runtime more
like a game by introducing other functions like buying and
selling Kitties. In order to achieve this, we'll first need to enable users
to update the price of their Kitty. Then we can add functionality to enable users to transfer, buy and breed Kitties.

## Steps

### 1. Set a price for each Kitty

#### A. Updating a stored struct

Every Kitty object has a price attribute that we have set to 0 as default.

> The default value for `price` is set to `0` in the `create_kitty` function. This inforrmation we'll use to check if the Kitty is for sale.

To update the price of a Kitty, we'll need to:

- Pull down the Kitty object.
- Update the price.
- Push it back into storage.

Changing a value from an existing struct would be written in the following way:

```rust
let mut object = Self::get_object(object_id);
object.value = new_value;

<Objects<T>>::insert(object_id, object);
```

:::note
Rust expects you to declare a variable as mutable (using the `mut` keyword) whenever its value is going to be updated.
:::

#### B. Checking Kitty owner

As we create functions which modify objects in storage, we
should check that only the appropriate users are successful when calling those functions.

For modifying a Kitty object, we'll first need to get the `Hash` value of the owner of the
Kitty to ensure that it's the same as the sender.

> `KittyOwner` stores a mapping to an `Option<T::AccountId>` but that
> given Hash may not point to a generated and owned Kitty yet.
> This means, whenever we fetch the owner of a Kitty, we need to
> resolve the possibility that it returns `None`. This could be
> caused by bad user input or even some sort of problem with our
> runtime. Checking will help prevent these kinds of problems.

An ownership check for our pallet will look something like this:

```rust
let owner = Self::owner_of(object_id).ok_or("No owner for this object")?;

ensure!(owner == sender, "You are not the owner");
```

#### C. Sanity checks

In a similar vain of checking permissions, we also need to ensure that our runtime performs regular sanity
checks to mitigate any risk that things go wrong &mdash; like users flooding the chain with heavy transactions or anything that could break the chain.

If we're creating a function geared to update the value of an object in storage,
the first thing we better do is make sure the object exists at all.
By using `ensure!`, we can create a safe guard against poor user input, whether malicious or unintentional. For example:

```rust
ensure!(<MyObject<T>>::exists(object_id));
```

:::tip Your turn!
You now have all the tools necessary to write the `set_price` dispatchable function. Remember, "_verify first, write last_": be sure to perform the appropriate checks before you modify storage, and don't assume that the user is giving you
good data to work with!
:::

### 2. Transfer a Kitty

Our pallet's storage items already handles ownership of Kitties
&mdash; this means that all our `transfer_Kitty` function needs to do is update following storage items:

- `KittyOwner`: to update the owner of the Kitty.
- `OwnedKittiesArray`: to update the owned Kitty map for each acount.
- `OwnedKittiesIndex`: to update the owned Kitty index for each owner.
- `OwnedKittiesCount`: to update the amount of Kitties a account has.

You already have the tools you'll need to create the transfer functionality from [section 1](#1-set-a-price-for-each-kitty). The main difference in how we will
create this functionality for our runtime is that it will have two parts to it:

- **A dispatchable function called `transfer()`:** this is what's exposed by your pallet.
- **A private function called `transfer_from()`:** this will be a helper function called by `transfer()` to handle all storage updates when transferring a Kitty.

Separating the logic this way makes the private `transfer_from()` function reusable
by other dispatchable functions of our pallet, without needing to rewrite the same logic over and over again. In our case, we're going to reuse it for
the `buy_Kitty` dispatchable we're creating in the next section.

:::tip Your turn!
**Tip:** Start by writing the `transfer_from()` function, making sure you've included all the necessary state changes. Remember to "verify first, write last"!
:::

### 3. Buy a Kitty

#### A. Check a Kitty is for Sale

We can use the `set_price()` function to check if the Kitty is for sale. Remember that we said a Kitty with the
price of 0 means it's not for sale? Easy enough then: write this check by simple using `ensure!()`.

#### B. Making a Payment

In the previous step, we've added the functions necessary to transfer the ownership of our
Kittys. But we never actually specified a currrency associated to our pallet, in the event that the Kitty was bought or sold.

In this step we'll learn how to use FRAME's Currency trait to adjust account balances
using its very own [`transfer` method][transfer-currency-rustdocs]. It's useful to understand why it's important to use the `transfer` method in particular and how we'll be accessing it:

- The reason we'll be using it is to ensure our runtime has the same understanding of currency throughout the pallets
  it interacts with. The way that we ensure this is to use the `Currency` trait
  from [`frame_support`][frame-balances-rustdocs].

- Conveniently, it handles a
  `Balance` type, making it compatible with `pallet_balances` which we've been
  using from our pallet's configuration trait. Take a look at how the `transfer`
  function we'll be using is structured (from the Rust docs):

```rust
fn transfer(
    source: &AccountId,
    dest: &AccountId,
    value: Self::Balance,
    existence_requirement: ExistenceRequirement
) -> DispatchResult
```

In order to use it, we'll need to import `Currency` and `ExistenceRequirement` from `frame_support`.

:::note "Verify First, Write Last" checklist

Now that we've gone over the necessary components for our `buy_Kitty` dispatchable function,
you have everything you need to know to put it together &mdash; you're in the drivers seat! Use the following points as
a guide to write `buy_Kitty` from scratch.

**Basic sanity checks**

- it will take 3 arguments: `origin`, `Kitty_id` and `max_price`
- check that `Kitty_id` corresponds to a Kitty in storage
- check that the Kitty has an owner

**Check if purchasing a Kitty is authorized**

- check that the account buying the Kitty doesn't already own it
- check that the price of the Kitty is not zero (if it is, throw an error)
- check that the Kitty price is not greater than `ask_price`

**Update storage items**

- use the `transfer` method from the `Currency` trait to update
  account balances
- use our pallet's `transfer_from` function to change the ownership
  of the Kitty from `owner` to `sender`
- update the price of the Kitty to the price it was sold at
  :::

:::tip Your turn!
Write the `buy_Kitty` dispatchable.

**Hint #1**: Make sure you include all the relevant checks &mdash; refer back to the checklist above!

**Hint #2**: Make sure to import any other dependencies you might need
:::

### 4. Breed Kitties

The logic behind breeding two Kitties is to multiply each corresponding DNA segment from two Kitties,
which will produce a new DNA sequence. Then, that DNA is used when minting a new Kitty. This piece is written for you to use. Here's what that
looks like in code:

```rust
    let Kitty_1 = Self::Kitty(Kitty_id_1);
    let Kitty_2 = Self::Kitty(Kitty_id_2);

    let mut final_dna = Kitty_1.dna;
    for (i, (dna_2_element, r)) in Kitty_2
        .dna
        .as_ref()
        .iter()
        .zip(random_hash.as_ref().iter())
        .enumerate()
    {
        if r % 2 == 0 {
            final_dna.as_mut()[i] = *dna_2_element;
        }
    }
```

For the `breed_kitty` dispatchable, you will need to handle the user inputs of Kitty IDs to breed. Then call the `mint()` function to create the new Kitty.

:::tip Your turn!
Write `breed_Kitty` following the template file for this part of the tutorial.
:::

### 5. Update `runtime/lib.rs` and interact with your Kitties

If you've completed all of the preceding parts and steps of this tutorial, you're
all geared up to run your chain and start interacting with all the new capabilities of your Kitties pallet.

Build and run your chain using the following commands:

```bash
cargo build --release
./target/release/kitties-node --dev
```

Now check your work using the Polkadot-JS Apps UI just like [we did in the previous part](/docs/Tutorials/Kitties/dispatchables-and-events#5-testing-with-polkadotjs-apps). Once your chain is running and connected to the PolkadotJS Apps UI, perform these manual checks:

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

- Connect your chain to the front-end template
- Customize the template using PolkadotJS API
- Interact with your chain

[transfer-currency-rustdocs]: https://crates.parity.io/frame_support/traits/tokens/currency/trait.Currency.html#tymethod.transfer
[frame-balances-rustdocs]: https://crates.parity.io/frame_support/traits/tokens/currency/trait.Currency.html
[polkadotjs-ui]: https://polkadot.js.org/apps/#/explorer
