# Turn-key Private Blockchain Network

## Project Description

Cere Network is providing a turn-key Private/Permissioned/Standalone blockchain network which can be readily integrated by any enterprise. Since it’s built with Substrate, this network can be integrated into any Polkadot/Substrate based Layer 1 network to serve as a secondary chain.   

This turn-key network intends to abstract the implementation complexity for businesses, as well as providing a ready-made package to optimize for a higher level of security, privacy, and performance, and to serve as a template or base-implementation of a highly customizable and performant enterprise specific blockchain network.   

Furthermore, any network built from or derived from this project will also be able to use derivative assets to represent real-world value transfers on-chain (e.g. micropayments, discount vouchers, loyalty points, etc), as well as being able to programatically issue these assets between user and application wallets.   

Below we have the overview of all the key features that this project will support:   

* A set of turn-key substrate based packaging and tools that simplifies the customization, configuration, testing, and deployment of such a blockchain network.
* Pre-built solutions to create/assign/transfer derivative assets in business to consumer use cases.
* Pre-configured and optimized for feeless transactions and performance.
* Creation of custom derivative assets and automate the transfer to/from user wallets to app wallets by any/app brand.
* Optimization of batch user onboarding and transaction processing for higher throughput situations needed for consumer apps/sites.

### Repository Hierarchy:
```
├── Private Standalone Network (PSN) Node [link](https://github.com/Cerebellum-Network/private-standalone-network-node)
│   ├── ./node ["PSN Node"]
│   ├── ./scripts [Packaging & Deployment Scripts]
│   ├── ./pallets [PSN Pallets]
│   │	    └── ./pallets/cere-ddc [Transfer Data Pallet (future)]
│   │    	        └── Cere DDC service connector
│   └── ./runtime [PSN Runtime Module]
│    	    └── Included custom Cere DDC Pallet
│
└── Cere Enterprise Smart Contracts [link](https://github.com/Cerebellum-Network/cere-enterprise-smart-contracts)
    └── ./cere01 [Enterprise Derivative Assets]
         └── ./cere01/specification [CERE01 Standard definition]
         └── ./cere01/lib.rs [Implementation, Tests]
```

There will be three primary directories in this repository:   

**Node**: A blockchain node is an application that allows users to participate in a blockchain network. Substrate-based blockchain nodes expose a number of capabilities like, Networking, Consensus, RPC Server.   

**Runtime**: The terms "runtime" and "state transition function" are analogous - they refer to the core logic of the blockchain that is responsible for validating blocks and executing the state changes they define. The Substrate project in this repository uses the FRAME framework to construct a blockchain runtime. FRAME allows runtime developers to declare domain-specific logic in modules called "pallets".   

**Pallets**: The runtime in this project is constructed using many FRAME pallets that ship with the core Substrate repository and a template pallet that is defined in the pallets directory. A FRAME pallet consists of a number of blockchain primitives namely, Storage, Dispatchables, Events, Errors and Trait.   

## Development Roadmap 

### Milestone 1

* **Estimated Duration:** 5 weeks 	
* **Costs:** 0.8 BTC	
* **Main Goal:** Basic functionality: Private node, Smart Contract implementation and setup guide.	


| Number | Deliverable | Specification |
| ------------- | ------------- | ------------- |
| 1. | [Documentation](https://github.com/Cerebellum-Network/private-standalone-network-node#documentation) and [basic tests](https://github.com/Cerebellum-Network/turnkey-private-blockchain-network/blob/dev/docs/deployment.md) | We will provide README files inside repositories with instructions of how to build, deploy and test. |
| 2.  | [Ink! based Smart Contract Standard](https://github.com/Cerebellum-Network/cere-enterprise-smart-contracts/blob/master/cere01/specification.md) | We are introducing a new smart contract standard which allows assets adaptable for real businesses to be programmatically created, managed, owned, transferred, and traded. It provides a template for establishing a foundation to capture common enterprise utility, and can be easily extended. This standard is purposefully being built on top of [Parity’s ink! Smart contract framework](https://github.com/paritytech/ink). |
| 2a.  | [Enterprise Derivative Assets](https://github.com/Cerebellum-Network/cere-enterprise-smart-contracts/blob/master/cere01/specification.md) | Derivative Asset support for the enterprise needs, with attributes such as expiration, limit on transfers, longitudinal unlocking, redemptions, etc |
| 2b.  | [Manual Direct Wallet Transfer](https://github.com/Cerebellum-Network/private-standalone-network-node/blob/dev/docs/direct_wallet_transfer.md) | Support for most Substrate/Polkadot based wallet applications. Smart Contract transfer function allows for the directly wallet-signed transfer of assets from one application/user address to the other. |
| 2c.  | [Programmatic Asset Transfer](https://github.com/Cerebellum-Network/cere-enterprise-smart-contracts/blob/master/cere01/lib.rs#L94) | Smart Contract transfer function allows for the programmatic/automated transfer of tokens from one application/user via smart contract to the other. |
| 2d.  | [Asset Restrictions](https://github.com/Cerebellum-Network/private-standalone-network-node/blob/dev/docs/asset_restrictions.md) | Support for the locking of assets by time or by issuer permission, support for expirations and potentially invalidations. |
| 3.  | [Smart Contract Tests](https://github.com/Cerebellum-Network/cere-enterprise-smart-contracts/blob/master/cere01/lib.rs#L190) | The Smart Contract implementation will include unit tests, we will be using the off-chain test environment that ink! provides. |

### Milestone 2

* **Estimated Duration:** 4 weeks 	
* **Costs:** 1.0 BTC

| Number | Deliverable | Specification |
| ------------- | ------------- | ------------- |
| 1.  | [Supporting Fee Abastraction](https://github.com/Cerebellum-Network/private-standalone-network-node/blob/dev/docs/fee_abstraction.md) | This is an important feature to allow enterprises to conduct value transfers between app/user accounts without worrying about fees. This was moved from milestone 1 to milestone 2 to allow more testing with a couple of different approaches. |
| 2.  | [Deployment packaging](https://github.com/Cerebellum-Network/private-standalone-network-node/blob/dev/docs/deployment.md) | At minimum the docker container or even the entire script that packages the container with the latest code from Substrate will also run on CI to test the deliverables of the milestone. |
| 3.  | [Testing](https://github.com/Cerebellum-Network/private-standalone-network-node/blob/dev/docs/tests.md) | Repositories including the  deployment and test sections for instructions and scripts to help contributors to package, deploy, run, test. |
| 4.  | [(optional) Batch processing](https://github.com/Cerebellum-Network/private-standalone-network-node/blob/dev/docs/batch_processing.md) | Allowing an app to optimize for creating asset transfers or data events to a batch of users at once, this would be a very nice to have from our practical experience. |
| 5.  | [Tutorial](https://docs.google.com/document/d/11B_i54-5RLfI66raUKmVXx4_YwQd5oee6ZVaLDMo9c0/edit?ts=5ffc3ee8#heading=h.52fwlvlu3qmg) | Cere will provide written documentation as well as a video tutorial on how to integrate and use Cere’s turnkey private blockchain networks for applications to showcase the ease of use. |
| 6.  | [Article](https://docs.google.com/document/d/1dzUrG7QvZ2fVzy3o1yh53NJkO01FMMh3-x42XtD_3s8/edit#heading=h.sorhd6104qu) | The main topic/theme: “...Cere Network is providing a turn-key permissioned standalone blockchain network which can be readily integrated by any enterprise. Since it’s built with Substrate, this network can be potentially integrated into any Polkadot/Substrate based Layer 1 network to serve as a secondary chain. Furthermore, any network built from or derived from this project will also be able to use derivative assets to represent real-world value transfers on-chain (e.g. micropayments, discount vouchers, loyalty points, etc), as well as being able to programatically issue these assets between user and application wallets... |  

## Documentation
This repository consist of a [new smart contract standard](https://github.com/Cerebellum-Network/cere-enterprise-smart-contracts/blob/master/cere01/lib.rs) which allows assets adaptable for real businesses to be programmatically created, managed, owned, transferred, and traded. It provides a template for establishing a foundation to capture common enterprise utility, and can be easily extended. This standard is purposefully being built on top of Parity’s ink! Smart contract framework.

## Quick Start Guide
See [Quick Start Guide](docs/quick-start.md)

## Tutorial
See [Private Network Tutorial](docs/tutorial.md)

## Testing key functionalities
* [Enterprise Derivative Assets](./docs/derivative_assets.md)
* [Manual Direct Wallet Transfer](./docs/direct_wallet_transfer.md)
* [Programmatic Asset Transfer](./docs/automated_token_transfer.md)
* [Asset Restrictions](./docs/asset_restrictions.md)


## Packaging and deployment
See [Packaging and deployment](./docs/deployment.md)

## Contributing
See [Contributing](./docs/contribution.md)

## Running tests
See [running tests](./docs/tests.md)
