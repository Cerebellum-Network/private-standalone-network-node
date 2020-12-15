# Enterprise Derivative Assets

### Description 
 Derivative Asset support for the enterprise needs, with attributes such as expiration, limit on transfers, longitudinal unlocking, redemptions, etc.
 
## How to build and deploy Smart Contract to the Network   
### 1. Create Smart Contract artifacts
Follow [this](./create_sc_artifacts.md) link to create Smart Contract artifacts
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
3. Any hash like: "0x77f017e167e070864b150a4f9b8cd416bf89c2af8eaa3f2e228de8ad13e4dd23" 
4. Specify "endowment" = 2

No need to touch "hashStr" and "commentIn"

Click deploy.

#### That's it!
