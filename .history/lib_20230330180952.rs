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
            if value.len() > 50 {
                Err(Error:: BadString);
            }
            self.value.push(String::from(value););
            Ok(String::from(self.value.clone()))
        }

        /// A function to handle direct off-chain Query from users.
        /// Such functions use the immutable reference `&self`
        /// so WILL NOT change the contract state.
        #[ink(message)]
        pub fn get_final(&self) -> Vec<String> {
            Ok(self.value.iter().cloned().collect())
        }
    }

}

