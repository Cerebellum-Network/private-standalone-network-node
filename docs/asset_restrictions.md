# Asset Restrictions 
Support for the locking of assets by time or by issuer permission, support for expirations and potentially invalidations.

### Supported features
* Distribution management
* Assets expiration

#### Distribution management
Transfer function adds the ability to transfer tokens between User accounts and Application and vice versa. In order to add more distribution wallets to enable application to user account transfers, `addDistributionAccount` function will allow to have multiple distribution wallets that can be used. In order to get a list of distribution accounts, `getDistributionAccounts` function should be used. 

#### Assets expiration
A possibility to allow the issuing of vouchers and adds a time limit for an asset (expiration date) Expiration is associated with the asset at the time of issuance. In order to issue restricted asset, `issueRestrictedAsset` function should be used.
