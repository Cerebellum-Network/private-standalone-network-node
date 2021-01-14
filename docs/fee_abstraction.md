# Fee Abstraction support
Since we provide a solution for applications/businesses, we don't want application owner to take care of fees, specifically for user's wallet. 

During the contract execution we provide a possibility to refund amount of fees to be charged from the user's wallet.

You can use either Polkadot UI to check this or do it programmatically:
#### 1. Send assets from user's account to the application account with refund using Polkadot UI:

Use [Tutorial](./tutorial.md) to configure Node, run Polkadot UI and deploy Smart Contract.

Contract call for the `transfer` function consist of `transactionFee` parameter:
![transaction fee parameter](https://staticassetsshare.s3-us-west-2.amazonaws.com/Screenshot+from+2021-01-12+14-05-47.png)
Let's keep it 0 for now and click `Execute` button.

Here we can see estimated fees for this transaction:
![estimated amount](https://staticassetsshare.s3-us-west-2.amazonaws.com/Screenshot+from+2021-01-12+14-06-03.png)
You can find it above the `sending from my account` block. So, 125.0002 nano Unit will be charged.

Click `Cancel` and specify this exact amount of tokens to be transferred:
![specify refund fee](https://staticassetsshare.s3-us-west-2.amazonaws.com/Screenshot+from+2021-01-12+14-06-22.png)
Now we can click `Execute` button and `Sign and Submit` on the next window.

We are done!

#### 2. Send assets programmatically:

Let's create transaction object first:
```javascript
const transferObj = await this.contract.tx.transfer(
  anyValue,
  gasLimitAuto,
  destinationAccountPublicKey,
  amount,
);
```
Now we can get estimated fees using [helper](https://polkadot.js.org/docs/api/cookbook/tx/#how-do-i-estimate-the-transaction-fees):
```javascript
const info = await transferObj.paymentInfo(this.userKeyring);
const partialFee = info.partialFee.toBn();
```
After we got the value we can re-create `transferObject` like this:
```javascript
const transferObj = await this.contract.tx.transfer(
  anyValue,
  gasLimitAuto,
  destinationAccountPublicKey,
  amount,
  partialFee,
);
```

Now we'are ready to submit transaction to the network:
```javascript
transferObj.signAndSend(this.userKeyring, <callback>);
```
