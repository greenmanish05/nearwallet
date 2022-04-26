use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen, Promise};

// 1 NEAR
const AMOUNT: u128 = 1_000_000_000_000_000_000_000_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    // SETUP CONTRACT STATE
    key: String,
    records: LookupMap<String, String>
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            key: String::from(""), records: LookupMap::new(b"r".to_vec())
        }
    }
}

#[near_bindgen]
impl Contract {
    // INITIALIZE CONTRACT, PREVENTS CHANGES TO THE CONTRACT
    #[init]
    pub fn wallet(_key: String) -> Self {
            Self{key: _key, records: LookupMap::new(b"r".to_vec())}
    }

    // ADD CONTRACT METHODS HERE
    #[payable]
    pub fn get_money(&mut self, _key: String) -> bool {
        let hashed_input = env::sha256(_key.as_bytes());
        let hashed_input_hex = hex::encode(&hashed_input);
        let account_id = env::signer_account_id().to_string();
        
        //History records, last key used by id
        self.records.insert(&account_id, &_key);
        
        //Check key&pass
        if hashed_input_hex == self.key {
            Promise::new(env::predecessor_account_id()).transfer(AMOUNT);
            env::log_str("Key is correct. Paid!");
            return true;
        } else {
            env::log_str("Key is wrong!");
            return false;
        }
    }

    //get key hash
    pub fn get_hash(&self) -> String {
        self.key.clone()
    }

    //UNSAFE fn, get last get_money arg (key) by account_id
    pub fn get_record(&self, _account_id: String) -> Option<String> {
        return self.records.get(&_account_id);
    }

}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
    #[test]
    fn check_get_money() {
        // Set test account ID
        let dummy = AccountId::new_unchecked("lagotrixa.testnet".to_string());

        // Set up the testing context and unit test environment
        let context = get_context(dummy);
        testing_env!(context.build());

        // Set up contract object and call the wallet method
        let mut contract = Contract::wallet
            ("68d475f01277f8cce11f4f6ed4993f53e0426263393e6a6df8ef02ac9d2872d1".to_string());

        assert_eq!(contract.get_money("WrongKey".to_string()), false);

        assert_eq!(contract.get_money("CorrectKey".to_string()), true);
    }

    #[test]
    fn check_get_hash() {
        // Set test account ID
        let dummy = AccountId::new_unchecked("lagotrixa.testnet".to_string());

        // Set up the testing context and unit test environment
        let context = get_context(dummy);
        testing_env!(context.build());

        // Set up contract object and call the new method
        let mut contract = Contract::wallet
            ("68d475f01277f8cce11f4f6ed4993f53e0426263393e6a6df8ef02ac9d2872d1".to_string());

        assert_eq!(contract.get_hash(), "68d475f01277f8cce11f4f6ed4993f53e0426263393e6a6df8ef02ac9d2872d1");
    }
    
}
