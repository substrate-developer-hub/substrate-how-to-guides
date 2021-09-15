---
sidebar_position: 3
keywords: parachains
---
# How to use the scheduler pallet for storage migrations 
🎯
## Goal

Implement storage migration logic using Substrate’s scheduler pallet for non-core migrations.

## Use cases

- Removing old unused storage during a runtime upgrade
- Migrating storage unrelated to the core logic of the chain (if the upgraded logic does not depend on the new storage format).

## Overview
This guide outlines steps to schedule non-core storage or other runtime migrations using the scheduler pallet.

## Steps

### 1. Define extrinsics in the runtime 
Define extrinsics that you want to be scheduled. Ensure the extrinsics can only be called via root or a pallet origin. When scheduling the extrinsics from your pallet, use this origin.

```rust

#[pallet::call]
impl<T: Config> Pallet<T> {
    pub fn example_run_migration(
        origin: OriginFor<T>
    ) -> DispatchResult {
        ensure_root(origin)?;
        //This extrinsic is what you schedule in your migration code.
        //It contains the migration logic. 
    }
}

```
### 2. Add the Scheduler type to the Configuration trait.
You need to define the Scheduler trait in your Config trait to access the Scheduler pallet from your pallet. 
Traits in the Scheduler pallet require a defined Call type as a parameter. You can also define the Call type in your Config trait to match your on-chain Scheduler pallet. 

```rust
#[pallet::config]
pub trait Config: frame_system::Config {
    /// The Call type that is scheduled.
    type ScheduledCall: Parameter + Dispatchable<Origin = Self::Origin> + From<Call<Self>>;
    /// The Scheduler.
    type Scheduler: ScheduleNamed<Self::BlockNumber, Self::ScheduledCall, Self::Origin>;
}
```
### 3. Schedule the calls within  the  `on_runtime_upgrade` hook  
- use the [```schedule_named```](https://github.com/paritytech/substrate/blob/master/frame/scheduler/src/lib.rs#L404) method from the Scheduler pallet to trigger your migration extrinsic.
```rust
#[pallet::hooks]
impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
    fn on_runtime_upgrade() -> Weight {
    // Anything that needs to be executed after the runtime upgrade but before on_initialize().
        match T::Scheduler::schedule_named(
            id,
			DispatchTime::At(when),
			maybe_periodic,
			priority,
			RawOrigin::Root.into()
            Call::example_run_migration().into(),
        )
        {
            Err(err) => frame_support::print(err),
            _ => (),
        };

        0 // Calculate the weight of this function.
    }
}

```

:::tip
 Schedule the extrinsic  for the blocks after the migration executes. If it takes an unknown length of time to execute, set up a counter within the extrinsic to make sure that it stops once it hits a certain weight and then schedules itself again for the next block.
:::

### 4. Trigger the migrations.

Once you have defined the calls and the `on_runtime_upgrade` hook you can trigger the migrations by upgrading the runtime as described [here](https://substrate.dev/substrate-how-to-guides/docs/storage-migrations/migration-steps-polkadotjs)
## Examples
 - Calls scheduling in  the [democracy pallet](https://github.com/paritytech/substrate/blob/0f934e970501136c7370a3bbd234b96c81f59cba/frame/democracy/src/lib.rs#L1711)

## Resources
 - Scheduler pallet [implementation](https://github.com/paritytech/substrate/tree/0f934e970501136c7370a3bbd234b96c81f59cba/frame/scheduler)
- [Manual storage migration](./runtime-upgrades#2-choose-your-upgrade-approach)

