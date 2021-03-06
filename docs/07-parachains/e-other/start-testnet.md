---
sidebar_position: 4
keywords: parachains, testing
---

# Start a testnet on Rococo
_Testing on Rococo._
## Goal
Start up a local testnetwork on Rococo.
## Use cases

## Overview

## Steps

### 1. Start ALice

```bash
cargo run -- \
  --base-path /tmp/alice \
  --chain rococo-local \
  --alice \
  --port 30333 \
  --ws-port 9944 \
  --rpc-port 9933 \
  --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
  --validator
```

### 2. Start Bob

Use Alice's  local node identity as shown in the stdout logs to declare as a bootnode.

```bash
cargo run -- \
  --base-path /tmp/bob \
  --chain rococo-local \
  --bob \
  --port 30334 \
  --ws-port 9946 \
  --rpc-port 9934 \
  --validator \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
```

## Examples

## Resources

#### Rust docs

#### Knowledgebase

#### Other 

- [Polkadot Wiki on Using Rococo](https://wiki.polkadot.network/docs/build-parachains-rococo)