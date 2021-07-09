---
sidebar_position: 4
keywords: parachains
---

# How to start a collator node and add more collators
_ ... _

## Goal


## Use cases
Launching a parachain.

## Overview

## Steps

### 1. Collator selection
You are free to choose your method of collator selection. Common methods include stake voting/staking (see [Cumulus implementation](https://github.com/paritytech/cumulus/blob/master/pallets/collator-selection/src/lib.rs))  or directly assigning collators via committee or other origins such as democracy. There are also different ways to implement collators' incentives, for example [this one](https://github.com/PureStake/moonbeam/blob/master/pallets/parachain-staking/src/lib.rs).

### 2. Starting a collator node
Refer [here](https://substrate.dev/cumulus-workshop/#/en/3-parachains/1-launch?id=start-the-collator-node) to start and set up a collator node.

### 3. Adding collators
Refer to [this instruction](https://substrate.dev/cumulus-workshop/#/en/3-parachains/4-more-nodes?id=start-the-second-collator) to add more collators.



## Examples

## Resources
- [Parachain devops best practices](https://gist.github.com/lovelaced/cddc1c7234b883ee37e71cf4a1d63cac)
- [Recording of a dedicated DevOps for parachains office hour](https://drive.google.com/file/d/1-nQ_SI2XK6vxPQvORWuv68Yj0UDz5FrO/view) 
#### Rust docs
#### Knowledgebase 
