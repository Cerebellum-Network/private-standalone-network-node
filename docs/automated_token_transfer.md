# Automated Token Transfer
Smart Contract transfer function allows for the programmatic/automated transfer of tokens from one application/user via smart contract to the other.
### Example
```js
const contract = // .. initialize contract promise here
const appKeyring = // ... initialize
const gasLimitAuto = -1;
const zero = 0;
const transferObj = await contract.tx.transfer(
  zero,
  gasLimitAuto,
  destinationAccountPublicKey,
  amount,
  '',
  '',
);

transferObj
    .signAndSend(appKeyring, ({status}) => {
      if (status.isInBlock) {
        console.log('In block');
      } else if (status.isFinalized) {
        console.log(`Finalized. Hash: ${status.asFinalized.toHex()}`);
      }
})
```
