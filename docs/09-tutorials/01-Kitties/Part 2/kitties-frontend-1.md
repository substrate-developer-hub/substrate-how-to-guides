---
title: Front-end Outline
sidebar_position: 1
keywords: pallet design, intermediate, runtime
---

## Learning outcomes

:arrow_right: Connect your chain to the Substrate front-end template.


## Overview

In Part 1 we created all of the back-end portion of our Kitties application. In this part, it's time to 
build a user interface which can access and interact with our
custom storage items and functions. We'll be using the Front-end Template, a React app with some basic functionality, and the
Polkadot JS API to make RPC's to our chain's runtime.

You might be already wondering: what library will we use to actually render each unique Kitty? We'll be taking a closer 
look at how that all works but the short answer is we'll be using [David Revoy's](https://framagit.org/Deevad) [library for generating Cat avatars](https://framagit.org/Deevad/cat-avatar-generator). 

## Steps

### 1. Understanding the Front-end template

The first step of this tutorial is to familiarize yourself with the Substrate Front-end template. In this step we will go through an overview of what our React app will look like and the different components we'll be building. 

Start by [installing the Front-end Template][substrate-frontend-template]:

```bash
# Clone the repository
git clone https://github.com/substrate-developer-hub/substrate-front-end-template.git
cd substrate-front-end-template
yarn install
```
First, clone and build the front-end template with the following command and open it up in your favorite code editor:

```bash
git clone substrate-front-end-template.git 
```

You'll notice the following structure (we've only including the directories we care about for this tutorial):

```
substrate-front-end-template
|
...
|
+-- public
|   |
|   +-- assets              <-- Kitty avatar PNG files
|
+-- src                     <-- our React components and helper folders
|   |
|   +-- __tests__
|   |
|   +-- config
|   |
|   +-- substrate-lib       <-- wrapper around PolkadotJS API
|   |
|   AccountSelector.js
|   App.js
|   Balances.js
|   BlockNumber.js
|   Events.js
|   index.js
|   interactor.js
|   Metadata.js
|   NodeInfo.js
|   TemplateModule.js
|   Transfer.js
|   Upgrade.js
|
...
```

In a separate terminal, start an instance of `node-kitties` that you built in Part 1:

```bash
# Launch `node-kitties` from its directory.
cd kitties/
./target/release/node-kitties --dev --tmp
```

Now, in the same directory as where you installed the Front-end template, launch it:

```bash
yarn start
```

You should see a tab open up with the front-end template displaying basic features of your chain.

You'll notice it comes with a number of prebuilt features, including:

- A wallet to manage and create keys + accounts.
- An address book to get details about accounts.
- A transfer function to send funds between accounts.
- A runtime upgrade component to make easy updates to your runtime.
- A key/value storage modification UX.
- A custom transaction submitter.

### 3. Specifying Types

Our front-end needs to know the custom types our node exposes. To do this, we'll need to go into `src/config/types.json` and paste in the following lines:

```json
{
  "Gender": {
    "_enum": ["Male", "Female"]
  },
  "Kitty": {
    "dna": "[u8; 16]",
    "price": "Option<Balance>",
    "gender": "Gender",
    "owner": "AccountId"
  }
}
```

### 4. Sketching out our application components

[Substrate Frontend Template][substrate-frontend-template] components use PolkadotJS Apps and an 
RPC endpoint to communicate with a Substrate node. This allows us to use it
to read storage items, and pass in inputs to allows users to make extrinsics by calling our pallet's
dispatchable functions.

Let's sketch out what we'll want our UI to look like, using our node's capabilities to
guide our application design.

**Functionality overview**

1. **Create Kitty**: accounts can create new Kitties.
2. **View Kitties**: all Kitties are visible.
3. **Identify Kitty owners**: all Kitty owners are visible.

**Buttons**

1. Create Kitty
2. Set Kitty's price
3. Breed a Kitty
4. Buy Kitty
5. Transfer Kitty

We'll be building out 3 components to handle the functionality outlined above:

1. `KittyCards.js`: this will render a React component containing the Kitty, its relevant information and buttons to interact with it.
2. `KittyAvatar.js`: this will handle the logic to map Kitty DNA to the library of PNGs we're using create the graphical visual of each Kitty. 
3. `Kitties.js`: this will be what we render to `App.js`.

We've provide template code for each components to help you follow along in the next sections.

#### Querying storage

Here's a break down of how PolkadotJS API helps us read our runtime's storage items:

- `api.query.substrateKitties.{storageItem}`: we can use `api.query` to access our pallet instance as we've named it in our runtime.
- `api.query.substrateKitties.storageItem.map( (item) => item)`:to query a storage map, we must use `map`


## Next steps

- Build the Kitties.js component
- Build the KittyAvatar.js component
- Build the KittyCards.js Component

[substrate-frontend-template]: https://github.com/substrate-developer-hub/substrate-front-end-template
