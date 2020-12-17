# Enterprise Derivative Assets

### Description 
 Derivative Asset support for the enterprise needs, with attributes such as expiration, limit on transfers, longitudinal unlocking, redemptions, etc.
 
## How to build and deploy Smart Contract to the Network   
### 1. Create Smart Contract artifacts
Follow [this](./create_sc_artifacts.md) link to create Smart Contract artifacts or download directly the latest version: [metadata](https://github.com/Cerebellum-Network/derivative-asset-smart-contract/blob/master/example/metadata.json), [wasm](https://github.com/Cerebellum-Network/derivative-asset-smart-contract/blob/master/example/cere01.wasm).
### 2. Upload artifacts (wasm and metadata files) first

1. Go to polkadot [UI](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fcore-dev.cere.io%2Fsubstrate#/contracts)
2. Open Developer→Contracts
3. Click "Upload WASM" button
4. Specify WASM file and metadata.json
5. Click upload and select specific account to upload artifacts.

### 3. Deploy Smart Contract

After you uploaded wasm file you should see this item in Hash Codes in the UI

1. Click "Deploy" button
2. Specify "initialSupply" value
3. Specify "endowment" = 2

No need to touch "hashStr" and "commentIn"

Click deploy.

#### That's it!
