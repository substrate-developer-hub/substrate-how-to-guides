---
sidebar_position: 6
keywords: pallet design, intermediate, runtime
---

# Kitties front-end

_Build the custom frontend for our Substrate Kitties._

## Learning outcomes

:arrow_right: Connect your chain to the Substrate front-end template.

:arrow_right: Use PolkadotJS API to customize template.

:arrow_right: [WIP]

## Overview

In Part 1 we created all of the back-end portion of our Kitties application. In this part, it's time to 
build a user interface which can access and interact with our
custom storage items and functions. We'll be using the Front-end Template, a React app with some basic functionality, and the
Polkadot JS API to make RPC's to our chain's runtime.

You might be already wondering: what library will we use to actually render each unique Kitty? We'll be taking a closer 
look at how that all works but the short answer is we'll be using [David Revoy's](https://framagit.org/Deevad) [library for generating Cat avatars](https://framagit.org/Deevad/cat-avatar-generator).

## Steps

### 1. Setting up the front-end template

Start by [installing the Front-end Template][substrate-frontend-template]:

```bash
# Clone the repository
git clone https://github.com/substrate-developer-hub/substrate-front-end-template.git
cd substrate-front-end-template
yarn install
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

### 2. Sketching out the components

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

### 3. Querying storage

Here's a break down of how PolkadotJS API helps us read our runtime's storage items:

- `api.query.substrateKitties.{storageItem}`: we can use `api.query` to access our pallet instance as we've named it in our runtime.
- `api.query.substrateKitties.storageItem.map( (item) => item)`:to query a storage map, we must use `map`

For each storage item we're going to need to use React hooks to fetch and set the current state of our application. The storage items relevant here are:

- `AllKittiesCount`
- `AllKittiesArray`
- `Kitties` 
- `KittyOwners` 

Start by copying the [`Kitties.js` helper file](https://substrate.dev/substrate-how-to-guides/static/code/kitties-tutorial/Kitties.js) into
the `src` folder of the Front-end template.

Replace ACTION item #1 with:

```js
  const [kittyCnt, setKittyCnt] = useState(0);
  const [kittyHashes, setKittyHashes] = useState([]);
  const [kittyStructs, setKittyStructs] = useState([]);
  const [kittyOwners, setKittyOwners] = useState([]);
  const [fetchState, setFetchState] = useState(0);
  const [kitties, setKitties] = useState([]);
```

Now we need to give our application a way to get items in storage and pass them to other parts of our React app.
To fetch Kitty count, replace ACTION #2 with:

```js
const fetchKittyCnt = () => {
    api.query.substrateKitties.allKittiesCount(cnt => {
      const cntNum = cnt.isNone ? 0 : cnt.toJSON();
      setKittyCnt(cntNum);
    });
  };
```

To fetch all Kitties, replace ACTION #3 with:

```js
const fetchKittyHashes = () => {
    // Increment by 1 because KittyIndex start from 1 instead of 0
    const kittyIndices = [...Array(kittyCnt).keys()].map(el => el + 1);
    api.query.substrateKitties.allKittiesArray.multi(
      kittyIndices,
      hashes => {
        setFetchState(0);
        setKittyHashes(hashes.map(hash => hash.toJSON()));
      }
    );
  };
```

### 4. Adding our custom types to our front-end

One of the first things we'll need to do in order for Polkadot JS API to be able to properly decode our node's storage items, is to 
provide our front-end with the our runtime's custom types.

Inside the Front-end Template directory, navigate to the `src/config/types.json` file and paste the following code inside it:

```json
{
  "Gender": {
    "_enum": ["Male", "Female"]
  },
  "Kitty": {
    "id": "Hash",
    "dna": "Hash",
    "price": "Balance",
    "gender": "Gender"
  }
}
```

### 4. Rendering Kitties

The most interesting part of our Kitties frontend is the logic behind rendering unique Kitties based on our Kitty DNA values.

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
