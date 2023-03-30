#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod ExquisiteCorpse {

    use ink_prelude::{string::String, vec::Vec};
    use ink_storage::{traits::SpreadAllocate, Mapping, traits::PackedLayout, traits::SpreadLayout};
    use scale::{Decode, Encode, EncodeLike};

    #[derive(Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        BadString,
    }
    /// Type alias for the contract's result type.
    pub type Result<T> = core::result::Result<T, Error>;
    

    impl EncodeLike<Vec<Slot>> for Slot {}
    #[derive( PackedLayout, SpreadLayout, Encode, Decode, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Slot {
        pub started_on: u64,
        pub ended_on: u64,
    }
    
    #[ink(storage)]
    pub struct ExquisiteCorpse {
        value: Vec<String>,
    }

    impl ExquisiteCorpse {
        /// Constructor to initializes your contract
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { value.push(String::from("C'est ici que tout commence...");) }
        }

        #[ink(message)]
        pub fn add_phrase(&mut self, value: String) -> Result<String> {
            self.value.push(String::from(value););
            Ok(String::from(self.value.clone()))
        }

        /// A function to handle direct off-chain Query from users.
        /// Such functions use the immutable reference `&self`
        /// so WILL NOT change the contract state.
        #[ink(message)]
        pub fn get_eth_balance(&self, account: String) -> Result<String> {
            if !account.starts_with("0x") && account.len() != 42 {
                return Err(Error::InvalidEthAddress);
            }

            // get account ETH balance with HTTP requests to Etherscan
            // you can send any HTTP requests in Query handler
            let resp = http_get!(format!(
                "https://api.etherscan.io/api?module=account&action=balance&address={}",
                account
            ));
            if resp.status_code != 200 {
                return Err(Error::HttpRequestFailed);
            }

            let result: EtherscanResponse = serde_json_core::from_slice(&resp.body)
                .or(Err(Error::InvalidResponseBody))?
                .0;
            Ok(String::from(result.result))
        }
    }


    impl ExquisiteCorpse {
        #[ink(constructor)]
        pub fn default() -> Self {
            // Even though we're not explicitly initializing the `Mapping`,
            // we still need to call this
            ink_lang::utils::initialize_contract(|contract: &mut PhatToMeet| {
                contract.value = String::from("I like dogs");
                contract.map_slot = Mapping::default();
                //contract
                ()
            })
        }

        #[ink(message)]
        pub fn get_slots(&self) -> Vec<Slot> {
            let caller = Self::env().caller();
            let mut slots = Vec::new();
            for slot in self.map_slot.get(&caller).unwrap_or_default() {
                slots.push(slot);
            }
            slots

        }

        #[ink(message)]
        pub fn add_slot(&mut self, started_on: u64, ended_on: u64) -> Result<()> {
            Self::check_is_not_future(&self,&started_on)?;
            Self::check_start_is_not_smaller_than_end(&self, &started_on, &ended_on)?;
            Self::is_not_overlapping(&self, &started_on, &ended_on)?;
            let caller = Self::env().caller();
            let new_slot :Slot = Slot{
                started_on: started_on,
                ended_on: ended_on,
            };
            let mut new_slot_vec: Vec<Slot> = self.map_slot.get(&caller).unwrap_or_default();
            new_slot_vec.push(new_slot);
            
            self.map_slot.insert(&caller, &new_slot_vec);
            Ok(())                     
        }

        // Simply returns the current value of our `var`.
        #[ink(message)]
        pub fn get_value(&self) -> String {
            self.value.clone()
        }


        #[ink(message)]
        pub fn set_value(&mut self, value: String) -> Result<String> {
            self.value = value;
            Ok(String::from(self.value.clone()))
        }

        // slots functions
        fn check_is_not_future(&self, ts: &u64) -> Result<()> {
            let now = self.env().block_timestamp();
            if ts < &now {
                Err(Error:: IsNotFutureDate)
            } else {
                Ok(())
            }
        }        
        fn check_start_is_not_smaller_than_end(&self, ts_start: &u64, ts_end: &u64) -> Result<()> {
            if ts_start > &ts_end {
                Err(Error:: StartIsBiggerThanEnd)
            } else {
                Ok(())
            }
        }
        fn is_not_overlapping(&self, ts_start: &u64, ts_end: &u64) -> Result<()> {
            let caller = Self::env().caller();
            for slot in self.map_slot.get(&caller).unwrap_or_default() {
                if [&ts_start, &ts_end].iter().any(|ts| (slot.started_on..slot.ended_on).contains(ts)) {
                    return Err(Error:: SlappingRangeWithExist);
                }
            }
            Ok(())            
        }
    }
}

