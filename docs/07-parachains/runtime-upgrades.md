---
sidebar_position: 6
keywords: parachains
---

## On-Chain Upgrades

### General

The Relay Chain needs to be informed before the runtime upgrade of your chain. Cumulus provides functionality to notify the Relay Chain about the upcoming upgrade. 
 - Provide the hash of your upgrade to [authorize your upgrade](https://github.com/paritytech/cumulus/blob/d935b81e7010fcf5c5639e238c78d865c1d6ed67/pallets/parachain-system/src/lib.rs#L359);
 - Provide the actual code for the upgrade and therewith [enact your upgrade](https://github.com/paritytech/cumulus/blob/d935b81e7010fcf5c5639e238c78d865c1d6ed67/pallets/parachain-system/src/lib.rs#L369). 

If both steps are correct the Relay Chain will be notified that the new upgrade has been scheduled.


### Multiblock Upgrades

If your existing substrate chain has a very large state, which you are migrating between storage formats it might not be possible to run all of the runtime migrations within one block. There are a handful of strategies you can use to remedy this problem.
1. If the amount of storage items to be migrated  can feasibly be processed within two or three blocks you can run the migrations via the [scheduler pallet](https://github.com/paritytech/substrate/tree/master/frame/scheduler) to ensure they get executed regardless of block producer.
2. Instead of migrating all of the items automatically, use versioned storage and only execute migrations when storage values that are unupgraded are accessed. This can cause variance in transaction fees between users and could potentially result in more complex runtime code but if properly metered (weights are properly benchmarked) will ensure minimal downtime for migration.
3. If you must split your migrations among multiple blocks you can do it either on-chain or off-chain:
    - An on-chain multiblock migration will require custom pallet logic to be written which can either queue changes over time or use the scheduler pallet to migrate chunks of storage at a time. 
     - Instead of adding migration code to your runtime you can generate the migration manually off-chain and use multiple system.setStorage calls to add and remove storage items as necessary via an origin with root permission (for example democracy). If you are limited in the number of transactions you can make, you can batch multiple transactions to occur over time via the scheduler. 


#### Resources
[Cumulus Overview](https://github.com/paritytech/cumulus/blob/master/docs/overview.md)

[Substrate Migrations - general](https://github.com/apopiak/substrate-migrations)

[Substrate Runtime Migration Guide](https://hackmd.io/BQt-gvEdT66Kbw0j5ySlWw?view)



