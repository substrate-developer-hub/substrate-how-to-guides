---
title: Kitty Front-end Outline
sidebar_position: 1
keywords: polkadotjs api, frontend, intermediate, runtime
---

## Overview

In Part 1 we created all of the back-end portion of our Kitties application. In this part, it's
time to build a user interface which can access and interact with our custom storage items and
functions. We'll be using the Polkadot-JS API, and Substrate Front-end Template, a React app
wrapping Polkadot-JS API to make it more React friendly, to make RPC's to our chain's runtime.

In Part 2, there will only be two main sections: the first focussing on setting up the Front-end
Template and the second focussing on building custom React components that can interact with our
kitty-node.

We'll be using a [library for generating Cat avatars](https://framagit.org/Deevad/cat-avatar-generator),
licensed under CC-By 4.0 attribution. Thank you [David Revoy's](https://framagit.org/Deevad) for
making this available.

## Learning outcomes

:arrow_right: &nbsp; Connect your chain to the Substrate front-end template.

:::tip
In case you are stuck in following this Part of the tutorial, the complete front-end solution
is accessible at [`Substrate Front-end Template` repo `tutorials/kitties` branch](https://github.com/substrate-developer-hub/substrate-front-end-template/tree/tutorials/kitties).
:::

## Steps

### 1. Understanding the Front-end template

The first step of this tutorial is to familiarize yourself with the Substrate Front-end template.
In this step we will go through an overview of what our React app will look like and the different
components we'll be building.

Start by [installing the Front-end Template][substrate-frontend-template]:

```bash
git clone https://github.com/substrate-developer-hub/substrate-front-end-template.git
cd substrate-front-end-template
yarn install
```

You'll notice the following structure (we've only including the directories we care about for this
tutorial):

```
substrate-front-end-template
|
+-- public
|   |
|   +-- assets              <-- Kitty avatar PNG files
|
+-- src                     <-- our React components
|   |
|   +-- __tests__
|   |
|   +-- config              <-- where to specify our custom types
|   |
|   +-- substrate-lib       <-- lib to give access to PolkadotJS API 
|   |   |
|   |   +-- components      <-- contains TxButton, used throughout our application
|   |
|   ...
...
```

In a separate terminal, start an instance of `node-kitties` that you built in Part 1:

```bash
# Launch `node-kitties` from its directory.
cd kitties
./target/release/node-kitties --dev --tmp
```

Now, in the same directory as where you installed the Front-end template, launch it:

```bash
yarn start
```

You should see a tab open up with the front-end template displaying basic features of your chain.

Notice that it comes with a number of prebuilt features, each being rendered by the provided
components of the Front-end Template.

### 2. Specifying Types

An important starting point when setting up a custom front-end for a Substrate node is creating a
JSON file with all of the node's custom types. These are types that we created in our pallet that
the Polkadot JS API doesn't know about.

Learn more about [Extending types](https://polkadot.js.org/docs/api/start/types.extend/) in the
Polkadot JS API documentation.

In our case, we have two custom types we'll need to add: the `Gender` enum and the `Kitty` struct.

To do this, go into `src/config/types.json` and replace the whole file with the following:

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

### 3. Sketching out our application components

[Substrate Frontend Template][substrate-frontend-template] components use Polkadot-JS API and an
RPC endpoint to communicate with a Substrate node. This allows us to use it to read storage items,
and make extrinsics by calling our pallet's dispatchable functions. Before we get to that, let's
sketch out the different parts of our application.

We'll be building out a total of 3 components:

1. `Kitties.js`: this will render the Kitty pane, and contains the logics of fetching all kitties
information from the connecting Substrate node.

2. `KittyCards.js`: this will render a React card component containing a Kitty's relevant
information, avatar and buttons to interact with it.

3. `KittyAvatar.js`: this will handle the logic to map Kitty DNA to the library of PNGs we're using
to create unique Kitty avatars.

### 4. Polkadot-JS API Basics

Before moving on to the next section, we reccommend you read a little Polkadot JS API documentation
to understand the basics of how we will be querying storage and triggering transactions. Here are
some good resources:

- [Basics and Metadata](https://polkadot.js.org/docs/api/start/basics)
- [RPC queries](https://polkadot.js.org/docs/api/start/api.rpc)
- [Storage methods](https://polkadot.js.org/docs/substrate/storage) such as
`api.query.<module>.<method>` to access a pallet instance in a runtime
- [Extrinsics methods](https://polkadot.js.org/docs/substrate/extrinsics) such as
`api.tx.<module>.<method>` to trigger a transaction.

## Next steps

- Build the `Kitties.js` component
- Build the `KittyAvatar.js` component
- Build the `KittyCards.js` Component

[substrate-frontend-template]: https://github.com/substrate-developer-hub/substrate-front-end-template
