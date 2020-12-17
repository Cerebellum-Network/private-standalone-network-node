# Quick Start Guide

1. Clone the repository:
```bash
git clone git@github.com:Cerebellum-Network/private-standalone-network-node.git
```

2. Run this command from the root directory in order to up and run single blockchain node on your local machine and play with it:
```bash
docker-compose -f ./docker-compose.public.yml up -d
```
3. Now you should have standalone node running on your local machine and you can access it via [Polkadot Block viewer](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2Flocalhost%3A9944#/explorer). You should be able to see the basic stats:
![initial-screen](https://staticassetsshare.s3-us-west-2.amazonaws.com/Screenshot+from+2020-12-15+23-25-43.png)
4. Now, let's deploy Smart Contract to the Network using [this guide](https://github.com/Cerebellum-Network/private-standalone-network-node/blob/dev/docs/derivative_assets.md#how-to-build-and-deploy-smart-contract-to-the-network).
5. Check out the [list](./../README.md#testing-key-functionalities) of features which available for Cere Enterprise Smart Contract.

#### Enjoy!
