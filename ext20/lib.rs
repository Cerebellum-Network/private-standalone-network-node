#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod ext20 {
    #[cfg(not(feature = "ink-as-dependency"))]
    #[ink(storage)]
    pub struct Ext20 {
        /// The total supply.
        total_supply: Balance,
        /// The balance of each user.
        balances: ink_storage::collections::HashMap<AccountId, Balance>,
        data_str: Hash,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        value: Balance,
        hash: Hash,
    }

    // ACTION: Add an `Approval` event
    //         It should emit the following:
    //         * `owner` as an `AccountId`
    //         * `spender` as an `AccountId`
    //         * `value` as a `Balance`
    //         * `hashstr` as a `Hash`

    impl Ext20 {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance, hash_str: Hash) -> Self {
            let caller = Self::env().caller();
            let mut balances = ink_storage::collections::HashMap::new();
            let hash = hash_str;
            balances.insert(caller, initial_supply);

            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: initial_supply,
                hash: hash,
            });

            Self {
                total_supply: initial_supply,
                data_str: hash_str,
                balances,
            }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balance_of_or_zero(&owner)
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance, hashstr: Hash) -> bool {
            self.transfer_from_to(self.env().caller(), to, value, hashstr)
        }

        fn transfer_from_to(&mut self, from: AccountId, to: AccountId, value: Balance, _hashstr: Hash) -> bool {
            let from_balance = self.balance_of_or_zero(&from);
            if from_balance < value {
                return false
            }

            // Update the sender's balance.
            self.balances.insert(from, from_balance - value);
            self.data_str = _hashstr;

            // Update the receiver's balance.
            let to_balance = self.balance_of_or_zero(&to);
            self.balances.insert(to, to_balance + value);

            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value,
                hash: _hashstr,
            });

            true
        }

        fn balance_of_or_zero(&self, owner: &AccountId) -> Balance {
            *self.balances.get(owner).unwrap_or(&0)
        }

    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use ink_lang as ink;

        #[ink::test]
        fn new_works() {
            let contract = Ext20::new(777, "Thanh send test".to_string());
            assert_eq!(contract.total_supply(), 777);
        }

        #[ink::test]
        fn balance_works() {
            let contract = Ext20::new(100, "Thanh send test".to_string());
            assert_eq!(contract.total_supply(), 100);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 0);
        }

        #[ink::test]
        fn transfer_works() {
            let mut contract = Ext20::new(100, "Thanh send test".to_string());
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
            assert!(contract.transfer(AccountId::from([0x0; 32]), 10, "Thanh send test".to_string()));
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 10);
            assert!(!contract.transfer(AccountId::from([0x0; 32]), 100, "Thanh send test".to_string()));
        }
    }
}
