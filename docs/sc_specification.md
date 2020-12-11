# A Standard For Real-world App Assets On Ink!

### Table Of Contents
[Abstract](#abstract)

[Summary](#summary)

[Specification](#specification)

[Asset Creation](#asset-creation)

[Asset Distribution](#asset-distribution)

* [transfer]()

* [addDistributionAccount]()

* [getDistributionAccount]()

* [issueRestrictedAsset]()

[Utility Methods](#utility-methods)

* [name]()

* [symbol]()

* [decimals]()

* [totalSupply]()

* [balanceOf]()


[Test Cases](#test-cases)

* [newWorks]()

* [balanceWorks]()

* [transferWorks]()
	
* [getDistributionAccountWorks]()

* [addDistributionAccountWorks]()
	
* [getRestrictiveAssetWorks]()

* [issueRestrictiveAssetWorks]()
	
[Events](#events)


#### Abstract
An ink! based new smart contract standard which allows assets adaptable for real businesses to be programmatically created, managed, owned, transferred, and traded. It provides a template for establishing a foundation to capture common enterprise utility, and can be easily extended. This standard is built on top of Parity’s ink! Smart contract framework. 

#### Summary
A standard interface allows any network built from or derived from this standard will also be able to use derivative assets to represent real-world value transfers on-chain (e.g. micropayments, discount vouchers, loyalty points, etc), as well as being able to programatically issue these assets between user and application wallets.
Following are the key features:
* Derivative Assets: Derivative Asset support for enterprise needs, with attributes such as expiration, limit on transfers, redemptions, etc.
* Fee-less Transactions: Allow private enterprise use to conduct value transfers between app/user accounts without worrying about fees.

### Specification

#### Asset Creation
 **token_name**
 
Name of the derivative asset.

 **deployment account**
 
Specifies the user account to use for this deployment. Any fees will be deducted from this account.

 **contract_name**
 
A name for the deployed contract to help users distinguish. Only used for display purposes.


The deployment constructor information for this contract, as provided in the ABI.
total_supply
Total supply of the asset that we mention during the Asset Creation (deployment).

Instructions for the deployment can be found here.



 **endowment**
 
The allotted endowment for this contract, i.e. the amount transferred to the contract upon instantiation.


 **max gas allowed (M)**
 
The maximum amount of gas that can be used by this deployment, if the code requires more, the deployment will fail.


#### Asset Distribution
 **transfer**
 
Transfers _value amount of tokens to address _to. The function SHOULD throw if the message caller’s account balance does not have enough tokens to spend. This adds the ability to transfer tokens between User accounts and Application and vice versa. This overrides the default behaviour of transfer function. This will be used for derivative asset use cases where we have Rewards, Discounts and Vouchers.
We’ve added account abstraction which let’s the application owners always pay for the fee in our transfer function.
```
pub fn transfer (&mut self, to: AccountId, value: Balance) -> bool

```





**addDistributionAccount**

Transfer function adds the ability to transfer tokens between User accounts and Application and vice versa. To add more distribution wallets to enable application to user account transfers, this function will allow us to have multiple distribution wallets that we can use.

```
pub fn add_distribution_account (&mut self, ds_address: AccountId) -> bool
```

 **getDistributionAccount**
 
This function gets the distribution account list.
```
pub fn get_distribution_account (&self) -> [AccountId; DS_LIMIT]
```

 **issueRestrictedAsset**
 
This function allows the issuing of vouchers and adds a time limit for an asset (expiration date) Expiration is associated with the asset at the time of issuance.


```
pub fn issue_restricted_asset ( &mut self, user_address: AccountId, value: Balance, has_time_limit: bool, time_limit: u64, ) -> bool
```


#### Utility Methods
 **name**
 
Returns the name of the token - e.g. "FrequentFlyerMiles".
OPTIONAL - This method can be used to improve usability, but interfaces and other contracts MUST NOT expect these values to be present.
```
pub fn name
```


 **symbol**
 
Returns the symbol of the token. E.g. “TOKEN”.
OPTIONAL - This method can be used to improve usability, but interfaces and other contracts MUST NOT expect these values to be present.
```
pub fn symbol
```


 **totalSupply**
 
Returns the total token supply.
```
pub fn total_supply
```

 **balanceOf**
 
Returns the account balance of another account with address owner / application owner.
```
pub fn balance_of
```


#### Test Cases
 **newWorks**
 
Test to check if the new instance of the Smart Contract successfully, the test runs by asserting an instance of the Smart Contract and initializing the values in the Constructor.

```
fn new_works()
```





 **balanceWorks**
 
Test to check if the balance_of function in the Smart Contract works successfully, the test runs by running the function on a set of addresses and asserting.


```
fn balance_works()
```


 **transferWorks**
 
Test to check if the transfer function in the Smart Contract works successfully.

```
fn balance_works()
```


 **getDistributionAccountWorks**
 
Test to check if the getDistributionAccount function in the Smart Contract works successfully.

```
fn getDistributionAccountWorks()
```


 **addDistributionAccountWorks**
 
Test to check if the addition of distribution accounts in the Smart Contract works successfully.

```
fn add_distribution_account_works()
```


 **getRestrictiveAssetWorks**
 
Test to check if getting the restrictive asset details in the Smart Contract works successfully.

```
fn get_restrictive_asset_works()
```
 **issueRestrictiveAssetWorks**
 
Test to check if the issuing of restrictive assets, issue_restrictive_asset function in the Smart Contract works successfully.

```
fn issue_restrictive_asset_works()
```



#### Events
**Transfer**

MUST trigger when tokens are transferred, including zero value transfers. A token contract which creates new tokens SHOULD trigger a Transfer event.

```
emit_event (Transfer { from: None, to: Some(caller), value: initial_supply, })
```

**Error**

MUST trigger when there’s an error in the execution of any smart contract function.

```
emit_event (ErrorDS { from: Some(from), to: Some(to), value, })
```


