---
sidebar_position: 6
keywords: pallet design, intermediate, runtime
---

# Part V: Kitties front-end
_[WIP] Build the custom frontend for our Substrate Kitties._

## Learning outcomes

:arrow_right: Connect your chain to the Substrate front-end template.

:arrow_right: Use PolkadotJS API to customize template.

:arrow_right: [WIP]

## Overview
Now that we have completed runtime development, it is time to build a user interface which can easily access and interact with our 
custom storage items and functions. We'll be using the frontend template, a React app with some basic functionality. [Install it][substrate-frontend-template] to be able to complete this part of the workshop. 

The Substrate frontend template comes with a number of prebuilt features, including:

- A wallet to manage and create keys + accounts.
- An address book to get details about accounts.
- A transfer function to send funds between accounts.
- A runtime upgrade component to make easy updates to your runtime.
- A key/value storage modification UX.
- A custom transaction submitter.

Since this course is primarily about runtime development, what you will learn in this section will not be as comprehensive as before, 
however it should empower you with the tools needed to extend your own knowledge and abilities.

## Steps

### 1. Sketching out the components
The [Substrate Frontend Template][substrate-frontend-template] comes with a set of custom components built in React.
These components use PolkadotJS Apps and an RPC endpoint to communicate with a Substrate node. This will allow
us to read our storage items, and pass in inputs to allows users to make extrinsics by calling our pallet's
dispatchable functions.

Let's sketch out what we'll want our Kitty frontend to look like, separating our node's capabilities by the React cards 
we'll need to render as well as the buttons each card will contain:

**Cards**
1. Create Kitty
2. View your Kitties
3. View all other Kitties and their owners

**Buttons**
1. Set Kitty's price
2. Breed a Kitty
3. Buy Kitty
4. Transfer Kitty


Structure of the frontend project:

```bash
folder structure with what we care about
and assets for kitty traits
```

### 2. Using PolkadotJS API to query storage

The Create Kitty card is simply an extrinsic and the frontend template already handles displaying events from our runtime.
Viewing Kitties requires that we display the values from the fields in the objects from our `Kitties` `StorageMap`. 

Here's a break down of how PolkadotJS API helps us do this:

- `api.query.substrateKitties.{storageItem}`: we can use `api.query` to access our pallet instance as we've named it in our runtime.
- `api.query.substrateKitties.storageItem.map( (item) => item)`:to query a storage map, we must use `map` 
- ...

Make sure you think about the props your passign in. We've done it for you.

### 3. Rendering Kitties
The most interesting part of our Kitties frontend is the logic behind rendering unique Kitties  based on our Kitty DNA values.

How does think work?

Essentially, each hex is split up into 6 segment where each segment corresponds to a physical Kitty trait.

This all happens in the `KittyCards.js` file ....

```js
 const populateKitties = () => {
```

And KittyAvatar does all the linking of .png files to DNA segments.

TODO.

## Next steps

[substrate-frontend-template]: https://github.com/substrate-developer-hub/substrate-front-end-template