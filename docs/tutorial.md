# Tutorial

1. Let's up and run single blockchain node in order to play with it. You can use Cerebellum Network private docker image:
```bash
docker-compose -f ./docker-compose.public.yml up -d
```
2. Now let's check if we can access to it using UI block viewer:
Follow this [link](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2Flocalhost%3A9944#/explorer). You should see something like this:
![initial-screen](https://staticassetsshare.s3-us-west-2.amazonaws.com/Screenshot+from+2020-12-15+23-25-43.png)
3. Now, let's deploy Smart Contract to the Network.
* Download [wasm file](https://github.com/Cerebellum-Network/derivative-asset-smart-contract/blob/master/example/cere01.wasm).
* Download [metadata file](https://github.com/Cerebellum-Network/derivative-asset-smart-contract/blob/master/example/metadata.json).
4. Deploy Smart Contract using [this](https://github.com/Cerebellum-Network/turnkey-private-blockchain-network/blob/dev/docs/derivative_assets.md#2-upload-artifacts-wasm-and-metadata-files-first) steps. You can use any account you want. Let's use Alice.
5. You should be able to see Smart Contract on the Developer -> Contracts page.
6. Now you can transfer just created derivative assets to the other account. Let's transfer some tokens from Alice account to Bob account.
7. You can find all features currently supported [here](./../README.md#supported-features).


#### Enjoy!
