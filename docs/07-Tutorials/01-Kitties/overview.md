---
sidebar_position: 1
keywords: pallet design, intermediate, runtime
---

# 😺 Substrate Kitties Workshop
_This is a 5 part series workshop that steps you through building a dapp for managing Substrate Kitties._

## Learning outcomes

:arrow_right: Write and integrate a custom FRAME pallet to your runtime.

:arrow_right: Use structs in storage and how to create and update storage items.

:arrow_right: Write extrinsics and helper functions.

:arrow_right: Use PolkadotJS API to connect a Substrate node to custom a front-end.

## Overview

Welcome to the Substrate Kitties workshop. This 5 part series workshop will teach you everything you need to know to build a blockchain designed to handle the creation and ownership management of Substrate Kitties. Before jumping into the next section, let's have a look at what we'll be doing. 

:::note 
You may find it useful to come back to this page
as you progress through this workshop &mdash; just to keep track of the bigger picture.
:::
### What we're building
In this tutorial, we'll intentionally keep things simple so that you can decide on how you'd like to improve your Substrate Kitties chain.  For the purposes of this workshop, Kitties really can only do the following things:

:smiley_cat: be created either by some original source or by being bred using existing Kitties.

:smirk_cat: be sold at a price set by their owner

:pouting_cat: be transferred from one owner to another

---
Bringing things down to a more granular level, this translates to the following application design:
1. [**Basic setup**](basic-setup). We'll need to spin up a Substrate node and create a custom pallet

2. [**Runtime storage**](basic-setup). We'll need a total of 9 storage items in our pallet to keep track of the amount of Kitties; their index; their owners and their 
owner account IDs.

3. [**Dispatchable functions**](extrinsics-and-events). We'll need a total of 5 dispatchable functions: `create`, `set_price`, `transfer`, `buy_kitty` and `breed_kitty`

4. [**Private functions**](create-kitties). We'll write 2 helper functions to handle randomness: `increment_nonce` and `random_hash`

5. [**Helper functions**](interacting-functions).We'll write 2 helper functions for our dispatchable functions: `mint` and `transfer_from`.

6. [**Rendering storage items**](kitties-frontend). We'll connect to a React front-end template and create the UI for each dispatchable call.

:::tip
Follow each step at your own pace &mdash; the goal is for you to learn and the best way to do that is to try it yourself!
Use the side panel to write your code as you follow along. Before moving on from one section to the next, make sure your pallet
builds without any error.
:::

<!-- ## Steps

### [1. Basic set-up](basic-setup) 

- Create a pallet and integrate it to your runtime
- Include a simple storage items to keep track of all Kitties
- Build and check your pallet

### [2. Create unique Kitties and their storage items](create-kitties)

- Write a struct to store details about our Kitties
- Implement the Randomness trait to create unique Kitties
- Use `StorageValue` and `StorageMap` to create the remainingn of your pallet's storage items 

### [3. Dispatchables and Events](extrinsics-and-events)
- Write a dispatchable that updates runtime storage using a helper function
- Write and use pallet Events

### [4. Interacting with your Kitties](interacting-functions)

- Write a dispatchable to set the price for a Kitty
- Create a transfer capabilities for a Kitty 
- Write a dispatchable to buy a Kitty
- Write a dispatchable to breed two Kitties

### [5. Viewing Kitties in a UI](kitties-frontend)

- Connect your chain to the Substrate front-end template
- Use PolkadotJS API to customize the frontend
- Interact with your chain 
 -->
