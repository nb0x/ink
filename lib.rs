#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod erc20 {
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::{
        traits::SpreadAllocate,
        Mapping,
    };

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Erc20 {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        allowances: Mapping<(AccountId, AccountId), Balance>,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    impl Erc20 {
        #[ink(constructor)]

        pub fn new(initial_supply: Balance) -> Self {
            ink_lang::utils::initialize_contract(|contract| {
                Self::new_init(contract, initial_supply)
            })
        }

        fn new_init(&mut self, initial_supply: Balance) {
            let caller = Self::env().caller();
            self.balances.insert(&caller, &initial_supply);
            self.total_supply = initial_supply;
            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: initial_supply,
            });
        }
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }
        #[ink(message)]
        pub fn balance_of(&self, account: AccountId) -> Balance {
            match self.balances.get(&account) {
                Some(value) => value,
                None => 0,
            }
        }
        fn allowance_of_or_zero(&self, owner: &AccountId, spender: &AccountId) -> Balance {
            self.allowances.get((owner, spender)).unwrap_or_default()
        }
        fn balance_of_or_zero(&self, owner: &AccountId) -> Balance {
            self.balances.get(owner).unwrap_or_default()
        }
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowance_of_or_zero(&owner, &spender)
        }
        #[ink(message)]
        pub fn transfer(&mut self, recipient: AccountId, amount: Balance) -> bool {
            let sender = self.env().caller();
            let sender_balance = self.balance_of(sender);
            if sender_balance < amount {
                return false;
            }
            let recipient_balance = self.balance_of(recipient);
            self.balances.insert(sender, &(sender_balance - amount));
            self.balances.insert(recipient, &(recipient_balance + amount));
            true
        }
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> bool {
            let caller = self.env().caller();
            let allowance = self.allowance_of_or_zero(&from, &caller);
            if allowance < value {
                return false;
            }
            self.transfer_from_to(from, to, value);
            self.allowances.insert((from, caller), &(allowance - value));
            true
        }
        fn transfer_from_to(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> bool {
            let from_balance = self.balance_of_or_zero(&from);
            if from_balance < value {
                return false;
            }
            self.balances.insert(from, &(from_balance - value));
            let to_balance = self.balance_of_or_zero(&to);
            self.balances.insert(to, &(to_balance + value));
            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value,
            });
            true
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn default_works() {
            let erc20 = Erc20::new(100);
            assert_eq!(erc20.total_supply(), 100);
        }
        
    }
}