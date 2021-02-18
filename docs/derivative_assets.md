# Enterprise Derivative Assets

### Description 
 Derivative Asset support for the enterprise needs, with attributes such as expiration, limit on transfers, longitudinal unlocking, redemptions, etc.
 
## How to deploy Enterprise Derivative assets via Smart Contract
### 1. Create Smart Contract artifacts
Checkout the latest Smart Contract artifacts [metadata.json](https://github.com/Cerebellum-Network/derivative-asset-smart-contract/blob/master/examples/cere01/metadata.json) and [cere01.wasm](https://github.com/Cerebellum-Network/derivative-asset-smart-contract/blob/master/examples/cere01/cere01.wasm) or follow the [link](./create_sc_artifacts.md) to build them from sources. 
### 2. Upload artifacts (wasm and metadata files) first

1. Go to Polkadot [Block Viewer](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2Flocalhost%3A9944#/contracts)
2. Open Developer→Contracts
3. Click "Upload WASM" button
4. Specify metadata.json and .WASM files
5. Click upload and select source account to upload artifacts. You can use pre-defined Alice account or any other with native tokens on balance.

### 3. Deploy Smart Contract

After you uploaded artifacts you will be able to see it in the Hash Codes section in the Polkadot Block Viewer

1. Click "Deploy" button
2. Specify `initialSupply` value. Let it be 1 000 000 000.
3. Specify `endowment` = 2

No need to touch `hashStr` and `commentIn`.

Click "Deploy" button.

#### That's it!

[< Back to the features list](./../README.md#testing-key-functionalities)
