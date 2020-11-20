#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod erc20 {
    #[cfg(not(feature = "ink-as-dependency"))]
    const DS_LIMIT: usize = 8;

    #[ink(storage)]
    pub struct Erc20 {
        /// The total supply.
        sc_owner: AccountId,
        /// The total supply.
        total_supply: Balance,
        /// The balance of each user.
        balances: ink_storage::collections::HashMap<AccountId, Balance>,
        /// List of distribution accounts
        ds_list: [AccountId; DS_LIMIT],
        /// Number of distribution accounts
        number_of_ds: u8,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        value: Balance,
    }

    #[ink(event)]
    pub struct ErrorDS {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        value: Balance,
    }

    // ACTION: Add an `Approval` event
    //         It should emit the following:
    //         * `owner` as an `AccountId`
    //         * `spender` as an `AccountId`
    //         * `value` as a `Balance`

    impl Erc20 {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let caller = Self::env().caller();
            let mut balances = ink_storage::collections::HashMap::new();
            let ds_list_temp = [caller; DS_LIMIT];

            balances.insert(caller, initial_supply);

            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: initial_supply,
            });

            Self {
                sc_owner: caller,
                total_supply: initial_supply,
                balances,
                ds_list: ds_list_temp,
                number_of_ds: 1,
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
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> bool {
            self.transfer_from_to(self.env().caller(), to, value)
        }

        #[ink(message)]
        pub fn get_distribution_accounts(&self) -> [AccountId; DS_LIMIT] {
            self.ds_list
        }

        #[ink(message)]
        pub fn add_distribution_account(&mut self, ds_address: AccountId) -> bool {
            let caller = self.env().caller();
            let saved_sc_owner = self.sc_owner;

            if caller != saved_sc_owner {
                return false;
            }

            let mut current_ds_list: [AccountId; DS_LIMIT] = self.ds_list;
            let number_of_ds_variable: u8 = self.number_of_ds;
            current_ds_list[usize::from(number_of_ds_variable)] = ds_address;
            self.ds_list = current_ds_list;
            self.number_of_ds = number_of_ds_variable + 1;
            true
        }

        fn transfer_from_to(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool {
            let ds_account_list = self.get_distribution_accounts();
            if !ds_account_list.contains(&from) {
                self.env().emit_event(ErrorDS {
                    from: Some(from),
                    to: Some(to),
                    value,
                });

                return false;
            }

            let from_balance = self.balance_of_or_zero(&from);
            if from_balance < value {
                return false;
            }

            // Update the sender's balance.
            self.balances.insert(from, from_balance - value);

            // Update the receiver's balance.
            let to_balance = self.balance_of_or_zero(&to);
            self.balances.insert(to, to_balance + value);

            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value,
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
            let contract = Erc20::new(888);
            assert_eq!(contract.total_supply(), 888);
        }

        #[ink::test]
        fn balance_works() {
            let contract = Erc20::new(888);
            assert_eq!(contract.total_supply(), 888);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 888);
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 0);
        }

        #[ink::test]
        fn transfer_works() {
            let mut contract = Erc20::new(888);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 888);
            assert!(contract.transfer(AccountId::from([0x0; 32]), 88), true);
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 88);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 800);
        }

        #[ink::test]
        fn get_distribution_accounts_works() {
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts");
            let contract = Erc20::new(888);
            let ds_account_list = contract.get_distribution_accounts();
            assert_eq!(ds_account_list.len(), DS_LIMIT);
            assert_eq!(ds_account_list[0], accounts.alice);
        }

        #[ink::test]
        pub fn add_distribution_account_not_owner_works() {
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts");
            let mut contract = Erc20::new(888);
            let ds_account_list = contract.get_distribution_accounts();

            assert!(contract.add_distribution_account(accounts.bob), true);
            assert_eq!(ds_account_list.len(), DS_LIMIT);
            assert_eq!(contract.number_of_ds, 2);
        }
    }
}
