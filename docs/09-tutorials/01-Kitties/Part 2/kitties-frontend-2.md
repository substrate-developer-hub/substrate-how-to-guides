---
title: Creating Custom Components
sidebar_position: 2
keywords: polkadotjs api, 
---

## Learning outcomes

:arrow_right: Connect your chain to the Substrate front-end template.

:arrow_right: Use PolkadotJS API to create custom React components.

## Overview

- Build the Kitties.js component
- Build the KittyAvatar.js component
- Build the KittyCards.js Component

## Steps

### 1. Create the Kitties.js component

This is the component that will get rendered by Apps.js. So it does the heavy lifting, with the help of KittyAvatar.js and KittCards.js.

:::note
Substrate-lib is a wrapper around Polkadot JS [api instance](https://polkadot.js.org/docs/api/start/create/)
:::

We need to retrieve the api from the polkadot.js keyring. This is why we use `useSubstrate`. Start by creating a file called `Kitties.js` and paste the following imports:

```js
import React, { useEffect, useState } from 'react';
import { Form, Grid } from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';

import KittyCards from './KittyCards';

const convertToKittyHash = entry =>
  `0x${entry[0].toJSON().slice(-64)}`;

const constructKitty = (hash, { dna, price, gender, owner }) => ({
  id: hash,
  dna,
  price: price.toJSON(),
  gender: gender.toJSON(),
  owner: owner.toJSON()
});

export default function Kitties (props) {
  const { api, keyring } = useSubstrate();
  const { accountPair } = props;

  const [kittyHashes, setKittyHashes] = useState([]);
  const [kitties, setKitties] = useState([]);
  const [status, setStatus] = useState('');
// snip
```

We use `useEffect` from `import React, { useEffect, useState } from 'react';` to listen for changes in our node's storage. There are two things our app needs to listen for: 

```js
// The kitties in existence.
  useEffect(subscribeKitties, [api, kittyHashes]);
// The total kitty count.
  useEffect(subscribeKittyCnt, [api, keyring]);
```

[TODO: Write-up]

#### Callback subscription functions

When the Kitty count changes what do we do? 

We need to look at the storge item for Kitties object: 

```js
const entries = await api.query.kitties.kitties.entries();
```

entries() is a Polkadot JS API function that gives us the entire storage map. If there's nothing, it passes in `None` which is like a promise. With entries() we get a key and a kitty object.

:::tip
entries() returns an array. You can get the first Kitty object in storage by doing: `entries[0][1].toJSON()`. 
If you wanted to query the first Kitty hash you would do:
`entries[0][0].toJSON()`
:::

#### Convert Kitty hash

Our polkadot js api uses the pallet name and storgae item for the first 64 bits and the unique storage item hash for the remaining 64 bits. We want to get rid of those and only keep the remaining bits which will be our kitty hash:

```js
const convertToKittyHash = entry =>
  `0x${entry[0].toJSON().slice(-64)}`;
```

To get all our kitty IDs we use :

```js
   const asyncFetch = async () => {
      unsub = await api.query.kitties.kittyCnt(async cnt => {
        // Fetch all kitty keys
        const entries = await api.query.kitties.kitties.entries();
        const hashes = entries.map(convertToKittyHash);
        setKittyHashes(hashes);
      });
    };
```

We use `entries` and need to re-request the Kitty object because 
        `const entries = await api.query.kitties.kitties.entries();`
does not support subscription functions. 

This is why we need:

```js
const asyncFetch = async () => {
      unsub = await api.query.kitties.kitties.multi(kittyHashes, kitties => {
        const kittyArr = kitties
          .map((kitty, ind) => constructKitty(kittyHashes[ind], kitty.value));
        setKitties(kittyArr);
      });
    };

    asyncFetch();
```

#### Clean up functions

In `asyncFetch` we're constantly listening to the Kitties storage. This is in relation to using Effects with Cleanup (see React docs). When the component is teared down, it will make sure that all remaining subscription functions are cleaned up:

```js
  // return the unsubscription cleanup function
    return () => {
      unsub && unsub();
    };
  };
```

Congratulations! What we've done up until here is prepare access to the Kitty object and other storage items for our React components.

We've filled an array of Kitties in `setKittyHashes(hashes);`

[TODO: Write-up]

### 2. Create KittyCards.js

Our KittyCards.js component will have three sections to it:

i. `TransferModal`: a modal that uses the `TxButton` component. 
ii. `KittyCard`: a card that renders the Kitty avatar using the `KittyAvatar.js` component as well as all other Kitty information (id, dna, owner, gender and price).
iii. `KittyCards`: a component that renders a grid for `KittyCard` (yes, singular!) described above. 

The part will step you through writing this component from scratch.

#### Preliminaries

As a preliminary step, create a new file called `KittyCards.js` and add the following imports:

```js
import React from 'react';
import { Button, Card, Grid, Message, Modal, Form, Label } from 'semantic-ui-react';

import KittyAvatar from './KittyAvatar'; 
import { TxButton } from './substrate-lib/components';
```

#### Outlining the TransferModal

Let's outline what the TransferModal will do.

The Substrate Front-end Template comes with a component called `TxButton` which is a useful way to include a transfer button that interacts with a node. This component will allow us to make an RPC call
into our node and trigger a signed extrinsic for the Kitties pallet. 

This is the `KittyCards.js` component's primary functionality. But how its built can be broken down into the following pieces:

- A "transfer" button exists, which upon being clicked opens up a modal
- This modal, we'll call "Kitty Transfer" is a `Form` containing (1) the Kitty ID and (2) an input field for a receiving adress
- It also contains a "transfer" and "cancel" button 

See the screenshot taken below for reference: 

<!-- ![image](/kitty-transfer-shot.png) -->

#### Setting up React hooks

The first thing we'll do is pass in the properties (or "props") and use React hooks from our React app. Do this by pasting in the following code snippet:

```js
const TransferModal = props => {
  const { kitty, accountPair, setStatus } = props;
  const [open, setOpen] = React.useState(false);
  const [formValue, setFormValue] = React.useState({});

  const formChange = key => (ev, el) => {
    setFormValue({ ...formValue, [key]: el.value });
  };
```
And now, close the React hook subscription function:

```js
  const confirmAndClose = (unsub) => {
    unsub();
    setOpen(false);
  };
```

#### Composing the modal

To recap: our Kitty Card has a "transfer" button that opens up a 
modal where a user can choose an address to send their Kitty to. That modal will have:
- a Title
- an input field for a Kitty ID
- an input field for an Account ID

In addition, it will have:
- a "cancel" button which closes the Transfer modal
- the `TxButton` React component to trigger the transaction

Here's what this looks like in code &mdash; paste this in to complete `TransferModal`:

```js
return <Modal onClose={() => setOpen(false)} onOpen={() => setOpen(true)} open={open}
    trigger={<Button basic color='blue'>Transfer</Button>}>

    // The title of the modal
    <Modal.Header>Kitty Transfer</Modal.Header>

    <Modal.Content><Form>
    // The modal's inputs fields
      <Form.Input fluid label='Kitty ID' readOnly value={kitty.id}/>
      <Form.Input fluid label='Receiver' placeholder='Receiver Address' onChange={formChange('target')}/>
    </Form></Modal.Content>
    <Modal.Actions>
    // The cancel button
      <Button basic color='grey' onClick={() => setOpen(false)}>Cancel</Button>
      // The TxButton component
      <TxButton
        accountPair={accountPair} label='Transfer' type='SIGNED-TX' setStatus={setStatus}
        onClick={confirmAndClose}
        attrs={{
          palletRpc: 'kitties',
          callable: 'transfer',
          inputParams: [formValue.target, kitty.id],
          paramFields: [true, true]
        }}
      />
    </Modal.Actions>
  </Modal>;
```

The next part of our KittyCards.js component is to create the part that renders the KittyAvatar.js component and the data passed in from the `kitties` props in Kitty.js.

### 3. Pass Kitty information to other components

#### The KittyCard "Card"  

We'll use React's `Card` component to create a card that render the Kitty avatar as well as the Kitty ID, DNA, gender, owner and price.

As you might have guessed, we'll use React props to pass in data to our KittyCard. Paste the following code snippet, reading through the comments to understand each code snippet:

```js
const KittyCard = props => {
  const { kitty, accountPair, setStatus } = props;
  const { id = null, dna = null, owner = null, gender = null, price = null } = kitty;
  const displayDna = dna && dna.toJSON();
  const isSelf = accountPair.address === kitty.owner;
```

Now let's make use of the previously imported `Card` component:

```js
return <Card>
    { isSelf && <Label as='a' floating color='teal'>Mine</Label> }
    // Render the Kitty Avatar
    <KittyAvatar dna={dna.toU8a()} />
    <Card.Content>
    // Display the Kitty ID
      <Card.Header style={{ fontSize: '1em', overflowWrap: 'break-word' }}>
        ID: {id}
      </Card.Header>
      // Display the Kitty DNA
      <Card.Meta style={{ fontSize: '.9em', overflowWrap: 'break-word' }}>
        DNA: {displayDna}
      </Card.Meta>
      // Display the Kitty ID, Gender, Owner and Price
      <Card.Description>
        <p style={{ overflowWrap: 'break-word' }}>
          Gender: {gender}
        </p>
        <p style={{ overflowWrap: 'break-word' }}>
          Owner: {owner}
        </p>
        <p style={{ overflowWrap: 'break-word' }}>
          Price: {price}
        </p>
      </Card.Description>
    </Card.Content>
```

Before closing the `<Card/>` component we want to render the TransferModal we privously built &mdash; only if the Kitty is transferrable by the acitve user account. Paste this code snippet in to handle this functionality:

```js
    // Render the transfer button using TransferModal
    <Card.Content extra style={{ textAlign: 'center' }}>{ owner === accountPair.address
      ? <TransferModal kitty={kitty} accountPair={accountPair} setStatus={setStatus}/>
      : ''
    }</Card.Content>
  </Card>;
```

#### Rendering the card

It's time to put all the pieces we've built together. In this function, we'll: 
- Check whether there's any Kitties to render and render a _"No Kitty found here... Create one now!"_ message if there aren't any
- If there are, render them in a 3 column grid

Have a look at the comments to understand the parts of this code snippet:

```js
const KittyCards = props => {
  const { kitties, accountPair, setStatus } = props;

// Check the number of Kitties
  if (kitties.length === 0) {
    return <Message info>
      <Message.Header>No Kitty found here... Create one now!&nbsp;
        <span role='img' aria-label='point-down'>👇</span>
      </Message.Header>
    </Message>;
  }
// Render Kitties using Kitty Card in a grid
  return <Grid columns={3}>{kitties.map((kitty, i) =>
    <Grid.Column key={`kitty-${i}`}>
      <KittyCard kitty={kitty} accountPair={accountPair} setStatus={setStatus}/>
    </Grid.Column>
  )}</Grid>;
};
```

And complete the component with:

```js
export default KittyCards;
```

### 4. Complete Kitties.js

Now that we've built all the pieces for our front-end application, we can piece everything together.

Go back to the incompleted Kitties.js file and paste this code snippet to render the KittyCard.js component inside a `<Grid/>`:

```js
return <Grid.Column width={16}>
    <h1>Kitties</h1>
    <KittyCards kitties={kitties} accountPair={accountPair} setStatus={setStatus}/>
```

Now we'll use the `<Form/>` component to render our application's `TxButton` component: 

```js
    <Form style={{ margin: '1em 0' }}>
      <Form.Field style={{ textAlign: 'center' }}>
        <TxButton
          accountPair={accountPair} label='Create Kitty' type='SIGNED-TX' setStatus={setStatus}
          attrs={{
            palletRpc: 'kitties',
            callable: 'createKitty',
            inputParams: [],
            paramFields: []
          }}
        />
      </Form.Field>
    </Form>
    <div style={{ overflowWrap: 'break-word' }}>{status}</div>
  </Grid.Column>;
```

### 5. Update App.js

In order to render Kitties.js, we need to as a row item to the `<Container/>` in App.js:

```js
<Grid.Row>
    <Kitties accountPair={accountPair} />
</Grid.Row>
```

Congratulations! You've finsished the front-end turorial! Now run `yarn start`, refresh your browser and you should be able to start interacting with your node.

## Next steps 

[TODO]