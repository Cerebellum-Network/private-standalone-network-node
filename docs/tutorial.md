# Private Network Tutorial

## Overview

This repo provides a description for setting up a ready-to-use private network that is designed to be easily integrated with any business application and Substrate-based blockchain to handle the in-app assets transfers between the application and its users, even facilitating peer-to-peer transfers  users.

This turn-key network intends to abstract the implementation complexity for businesses, as well as providing a ready-made package to optimize for a higher level of security, privacy, and performance, and to serve as a template or base-implementation of a highly customizable and performant enterprise-specific blockchain network. It presents a significant step towards easier adoption of blockchain technology for all who are part of the greater Web3 FoundationSubstrate ecosystem.

**Cere's Substrate-based private networks will feature:**
* A set of turn-key substrate-based packaging and tools that simplify the customization, configuration, testing, and deployment of such a blockchain network.
* Pre-built solutions to create/assign/transfer derivative assets in business to consumer use cases.
* Pre-configured and optimized for feeless transactions and performance.
* Creation of custom derivative assets and automate the transfer to/from user wallets to app wallets by any/app brand.
* Optimization of batch user onboarding and transaction processing for higher throughput situations needed for consumer apps/sites.

Next to that, Cere’s [custodial wallets](https://github.com/Cerebellum-Network/Cere-Identity-Custody-SDK) and Decentralized Data Clouds (DDCs) are designed to work in conjunction with this Network.

### Who is this repo meant for?

This repo aims to be beneficial to applications & developers who are looking to easily adopt  value adding blockchain technology in a turnkey manner.

How does this technology  benefit application developers?
* By facilitating Identity abstraction. Check [this link](https://github.com/Cerebellum-Network/Cere-Identity-Custody-SDK) out.
* By storing the data for each user in their individual ledger
* By assigning / distributing in-app assets via blockchain technology. Check [this link](https://github.com/Cerebellum-Network/private-standalone-network-node/blob/dev/docs/derivative_assets.md) out.

## How to start using Private Network

### 1. Set up the Cere Private Network using instructions below:

Option 1. Set up the Cere Private Network Node using `docker run` command:

```shell
docker run -d --name NAME_OF_YOUR_NODE -p 9944:9944 --entrypoint /usr/local/bin/node-template cerebellumnetwork/turnkey-private-blockchain-network --dev --ws-external
```

Option 2. Set up the Cere Private Network Node from `docker-compose` script:

* Clone this repo: `git clone https://github.com/Cerebellum-Network/test-scripts`
* Up and run node using script: `docker-compose up -d node`

### 2. Launch and deploy Assets that can be used in the app

* [Create Smart Contract artifacts](./derivative_assets.md#1-create-smart-contract-artifacts)
* [Upload artifacts to the Network](./derivative_assets.md#2-upload-artifacts-wasm-and-metadata-files-first)
* [Deploy Smart Contract to the Network](./derivative_assets.md#2-upload-artifacts-wasm-and-metadata-files-first)

### 3. Quickly test key functionalities.

Create a new wallet for the application.
In order to do this, you can use [this](https://www.youtube.com/watch?v=hhUZ40ZWqkE) tutorial. Save the wallet JSON ABI and passphrase for further steps.

Option 1: Test key functionalities by using the built-in standard tool.

Run Polkadot UI with the command:

```shell
docker run --rm -it --name polkadot-ui -e WS_URL=ws://localhost:9944 -p 81:80 jacogr/polkadot-js-apps:latest
```

Use [Testing Key Functionalities guide](https://github.com/Cerebellum-Network/private-standalone-network-node#testing-key-functionalities) to test manually.

Option 2: Test key functionalities by using a programmatic approach.

Once you created an application wallet, use [these](https://github.com/Cerebellum-Network/test-scripts) test scripts to create the user's wallet programmatically and transfer tokens to it.


