---
sidebar_position: 1
keywords: pallet design, intermediate, runtime
---

# 😺 Substrate Kitties

_This is a 2 part tutorial series that steps you through building a pallet for managing Substrate Kitties from scratch. Each part
could take between 1.5 and 3 hours to complete depending on your level of experience with Substrate._

![image](./kitties-tutorial.png)

## Introduction

Welcome to the Substrate Kitties tutorial. This 2 part tutorial designed to teach you everything you need to know to build a blockchain designed to handle the creation and ownership management of Substrate Kitties. Part 1 will cover how to build the Kitties pallet, including 
the functionality to interact with the Kitties we create, such as transfering, buying and setting prices. Part II will cover developing
the Front-end UI to interact with the blockchain we build in Part 1.

:::note
You may find it useful to come back to this page
as you progress through each part &mdash; just to keep track of the bigger picture.
:::

### What we're building

In Part 1 of this tutorial, we'll intentionally keep things simple so that you can decide on how you'd like to improve your Substrate Kitties chain later on. For the purposes of what we're building, Kitties really can only do the following things:

:smiley_cat: Be created either by some original source or by being bred using existing Kitties.

:smirk_cat: Be sold at a price set by their owner.

:pouting_cat: Be transferred from one owner to another.

**In Part 1, we'll go over:**

1. [**Basic setup**](/docs/Tutorials/Kitties/Part%201/basic-setup). We'll need to spin up a Substrate node and create a custom pallet

2. [**Runtime storage**](/docs/Tutorials/Kitties/Part%201/basic-setup). We'll need a total of 3 storage items in our pallet to keep track of the amount of Kitties; their owners and a Kitty object containing a single Kitty's information.

3. [**Dispatchable functions**](/docs/Tutorials/Kitties/Part%201/dispatchables-and-events).  We'll need a total of 5 dispatchable functions: `create`, `set_price`, `transfer`, `buy_kitty` and `breed_kitty`

4. [**Private functions**](/docs/Tutorials/Kitties/Part%201/create-kitties). We'll write 2 helper functions to handle randomness: `increment_nonce` and `random_hash`

5. [**Helper functions**](/docs/Tutorials/Kitties/Part%201/interacting-functions). We'll write 2 helper functions for our dispatchable functions: `mint` and `transfer_from`.

**In Part 2, we will:**

1. [**Create a UI for interacting with our pallet's storage items**](/docs/Tutorials/Kitties/Part%202/kitties-frontend). We'll connect to a React front-end template and create the UI for each dispatchable call.

2. [**Enhance our UX with Polkadot JS API**](/docs/Tutorials/Kitties/Part%202/kitties-frontend). 

### What we won't cover

The following fall outside the scope of this tutorial:

 - Writing tests for our pallet.
 - Declaring a configuration for the genesis of our chain.

 You can refer to the [how-to guides](/docs/intro) on how to do this once you've completed this tutorial series.

---

:::tip
Follow each step at your own pace &mdash; the goal is for you to learn and the best way to do that is to try it yourself!
Use the side panel to write your code as you follow along (coming soon). Before moving on from one section to the next, make sure your pallet
builds without any error.

You'll be writing most of the code yourself! Use the template files [here](https://github.com/substrate-developer-hub/substrate-how-to-guides/tree/main/static/code/kitties-tutorial)
to help you complete each part.
:::

## Learning outcomes

:arrow_right: Write and integrate a custom FRAME pallet to your runtime.

:arrow_right: Use structs in storage and how to create and update storage items.

:arrow_right: Write pallet extrinsics and helper functions.

:arrow_right: Use the PolkadotJS API to connect a Substrate node to a custom a front-end.

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
