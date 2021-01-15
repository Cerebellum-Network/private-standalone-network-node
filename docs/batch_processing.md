# Batch processing
In order to process huge amount of Transfer Asset and Send Data requests you can either use single transactions and handle it via the suggested architecture below or use Polkadot `utility.batch` method:
#### 1. Architecture approach:
![batch processing architecture](https://staticassetsshare.s3-us-west-2.amazonaws.com/Blank+diagram+(1).png)
#### 2. Using Polkadot `utility.batch` method:

```javascript
// construct a list of transactions we want to batch
const txs = [
  api.tx.balances.transfer(addrBob, 12345),
  api.tx.balances.transfer(addrEve, 12345),
  api.tx.staking.unbond(12345)
];

// construct the batch and send the transactions
api.tx.utility
  .batch(txs)
  .signAndSend(sender, ({ status }) => {
    if (status.isInBlock) {
      console.log(`included in ${status.asInBlock}`);
    }
  });
```
Source is [here](https://polkadot.js.org/docs/api/cookbook/tx/#how-can-i-batch-transactions).

Try using [Test Scripts](https://github.com/Cerebellum-Network/test-scripts) to check how it works on practise.
