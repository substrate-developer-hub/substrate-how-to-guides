---
sidebar_position: 3
keywords: parachains
---
# How to use scheduler pallet for storage migrations 
## Goal

Implement storage migration logic using Substrate’s scheduler pallet for non-core migrations.

## Use cases

- Removing old unused storage during a runtime upgrade
- Migrating storage unrelated to the core logic of the chain

## Overview
This guide outlines steps to schedule non-core storage or other runtime migrations using the scheduler pallet.

## Steps

### 1. Define extrinsics in the runtime 
Ensure the extrinsics can only be called via root.

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
### 2. Add the Scheduler trait to the Config trait.
```rust
#[pallet::config]
pub trait Config: frame_system::Config {
    type ScheduledCall: Parameter + Dispatchable<Origin = Self::Origin> + From<Call<Self>>;
    /// The Scheduler.
    type Scheduler: ScheduleNamed<Self::BlockNumber, Self::ScheduledCall, Self::Origin>;
}
```
### 3. Schedule the calls within  the  ```on_runtime_upgrade``` hook  
- use the [```schedule_named```](https://github.com/paritytech/substrate/blob/master/frame/scheduler/src/lib.rs#L404)  function.
```rust
#[pallet::hooks]
impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
    fn on_runtime_upgrade() -> Weight {
    // Anything that needs to be executed after the runtime upgrade but before on_initialize().
        if T::Scheduler::schedule_named(
            id,
			DispatchTime::At(when),
			maybe_periodic,
			priority,
			RawOrigin::Root.into()
            Call::example_run_migration().into(),
        ).is_err()
        {
            frame_support::print("LOGIC ERROR: on_runtime_upgrade/schedule_named failed");
        }
    }
}


```
:::tip
 Schedule the extrinsic  for the blocks after the migration executes. If it takes an unknown length of time to execute, set up a counter within the extrinsic to make sure that it stops once it hits a certain weight and then schedules itself again for the next block.
:::



### Manual storage migration

If you want to migrate storage manually:

 - Ensure you have the scheduler pallet available on your chain.
 - Use the root origin to schedule any changes to state using ```scheduler.scheduleNamed``` in the Apps UI Extrinsics tab.
 - Schedule changes for the blocks immediately after a ```system.setcode``` call is scheduled. 
 - Use ```system.set_storage``` and ```system.kill_storage``` calls.
 - Make sure that the scheduling fits within the PoV block size.
 - If the changes are very large, schedule them in advance over multiple blocks.

## Examples
 - Calls scheduling in  the [democracy pallet](https://github.com/paritytech/substrate/blob/0f934e970501136c7370a3bbd234b96c81f59cba/frame/democracy/src/lib.rs#L1711)

## Resources
 - Scheduler pallet [implementation](https://github.com/paritytech/substrate/tree/0f934e970501136c7370a3bbd234b96c81f59cba/frame/scheduler)


