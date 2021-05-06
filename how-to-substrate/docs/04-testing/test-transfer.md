---
sidebar_position: 1
---

# Testing a transfer function

_A basic overview to help you write out your checks and balances._

## Goal

Learn how to write tests and improve the correctness of your transfer function.

## Use cases

Testing a custom transfer function.

## Overview

Testing each function is an imporant part of developing pallets for production. This guide
steps you through best practices for writing a `transfer` function that passes all test cases.

## Steps

### 1. Outline the `transfer` function

A transfer function has two key elements: subtracting a balance from an account and adding that balance to another account. 

```rust
#[pallet::weight(10_000)]
pub (super) fn transfer(
    origin: OriginFor<T>,
    to: T::AccountId,
    #[pallet::compact] amount: T::Balance,
) -> DispatchResultWithPostInfo {
    let sender = ensure_signed(origin)?;

    Accounts::<T>::mutate(&sender, |bal| {
        *bal = bal.saturating_sub(amount);
    });
    Accounts::<T>::mutate(&to, |bal| {
        *bal = bal.saturating_add(amount);
    });
    Self::deposit_event(Event::<T>::Transfered(sender, to, amount))
    Ok(().into())
}
```

### 2. Check that the sender has enough balance

In a separate `tests.rs` file, write out the first test case:

```rust
#[test]
fn transfer_works() {
	new_test_ext().execute_with(|| {
		MetaDataStore::<Test>::put(MetaData {
			issuance: 0,
			minter: 1,
			burner: 1,
		});
        // Mint 42 coins to account 2.
        assert_ok!(RewardCoin::mint(Origin::signed(1), 2, 42));
        // Send 50 coins to account 3.
        asset_noop!(RewardCoin::transfer(Origin::signed(2), 3, 50), Error::<T>::InsufficientBalance);
```

#### Configure error handling

Replacing `mutate` with `try_mutate` to use `ensure!`. This will check that _bal >= amount_ and throw an error message if not:

```rust
Accounts::<T>::mutate(&sender, |bal| {
    ensure!(bal >= amount, Error::<T>::InsufficientBalance);
    *bal = bal.saturating_sub(amount);
    Ok(())
});
```

Run `cargo test` from your pallet's directory.

### 3. Check that sending account doesn't go below minimum balance

### 4. Check that both tests work together

Use `#[transactional]` to generate a wrapper around both checks:

```rust
#[transactional]
		pub(super) fn transfer(
/*--snip--*/
```

### 4. Handle dust accounts 

Make sure that sending and receiving accounts aren't dust accounts. Use `T::MinBalance::get()`: 

```rust
/*--snip--*/
    let new_bal = bal.checked_sub(&amount).ok_or(Error::<T>::InsufficientBalance)?;
    ensure!(new_bal >= T::MinBalance::get(), Error::<T>::BelowMinBalance);
/*--snip--*/
```


## Examples

- `reward-coin`

## Resources

- Rust docs `try_mutate`
- Rust docs `saturating_sub`
- Rust docs `assert_noop!`