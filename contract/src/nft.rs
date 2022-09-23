use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;

pub type TokenId = u32;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Token {
    pub owner_id: AccountId,
    pub token_id: TokenId,
    pub is_active: bool,
    pub tokendata: TokenData,
}

impl Token {
    // Transfer token to new owner who winner auction
    pub fn transfer(&mut self, new_owner_id: AccountId) {
        self.owner_id = new_owner_id.into();
    }
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenData {
    pub title: Option<String>,
    pub description: Option<String>,
    pub data: Option<String>,
    pub data_hash: Option<String>,
}
