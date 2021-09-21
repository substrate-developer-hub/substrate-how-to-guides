---
sidebar_position: 2
keywords: [storage migration, testing, runtime]
---

# Migration tests

_Verify first, write last ... and always test!_

## Goal

Write tests for a simple storage migration.

## Use cases

Testing a migration module inside a pallet.

## Overview

When writing a runtime migration module it is important to test it to avoid any critical issues caused by mangling storage items. This
guide provides a walk through of the tests to include for a simple storage migration of a single pallet, using the Nicks pallet
migration from the [Basic Storage Migration](./nicks-migration) guide as a reference.

## Steps

### 1. Write mock runtime

Refer to [this guide](../testing/basic-pallet-testing) to learn how to set up the dependencies for your test environment.

### 2. Specify unit tests

Derive the different tests that need to be included based on your pallet's functions.

For the Nicks pallet migration we need:

- `fn kill_name_should_work()`
- `fn force_name_should_work()`
- `fn normal_operation_should_work()`
- `fn error_catching_should_work()`

Using `asset_noop!(...)`, `assert_ok!(...)` and `assert_eq!(...)`, constructing one of these tests looks like this:

```rust
#[test]
	fn normal_operation_should_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Nicks::set_name(Origin::signed(1), b"Gav".to_vec(), None));
			assert_eq!(Balances::reserved_balance(1), 2);
			assert_eq!(Balances::free_balance(1), 8);
			assert_eq!(<NameOf<Test>>::get(1).unwrap().0.first, b"Gav".to_vec());

			assert_ok!(Nicks::set_name(Origin::signed(1), b"Gavin".to_vec(), None));
			assert_eq!(Balances::reserved_balance(1), 2);
			assert_eq!(Balances::free_balance(1), 8);
			assert_eq!(<NameOf<Test>>::get(1).unwrap().0.first, b"Gavin".to_vec());

			assert_ok!(Nicks::clear_name(Origin::signed(1)));
			assert_eq!(Balances::reserved_balance(1), 0);
			assert_eq!(Balances::free_balance(1), 10);
		});
	}
```

## Examples

- Migrating the Nicks pallet

## Resources

#### How-to guides

- [Migration steps](/migration-steps-polkadotjs)
- [Nicks migration](/nicks-migration)

#### Other

- [Fork-off Substrate tool](https://github.com/maxsam4/fork-off-substrate)
